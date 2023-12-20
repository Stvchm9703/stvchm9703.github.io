<script>
  import { Application, Text, Container, Graphics, getApp } from "svelte-pixi";
  import * as PIXI from "pixi.js";
  import Station from "./station.svelte";
  import { nodes } from "../data";
  import { onMount } from "svelte";
  import { tweened } from "svelte/motion";
  import { cubicOut } from "svelte/easing";
  let width = 1200,
    height = 1200;
  let center = { x: 1400 / 2, y: 1400 / 2 },
    radius = 1200 / 6 - 50;

  let currentLayerLv = 0;
  // const layer_angle = Object.groupBy(layer, "layer");

  const cos = (a) => Math.cos((Math.PI * a) / 180).toFixed(3);
  const sin = (a) => Math.sin((Math.PI * a) / 180).toFixed(3);

  const calc_xy = (layer_lv, id, countCap) => {
    // const countCap = layer[layer_lv];

    const angle = layer_lv !== 0 ? (360 / countCap) * ((id % countCap)) : 0;
    const position = {
      x: radius * layer_lv * cos(angle),
      y: radius * layer_lv * sin(angle),
      z: 1,
    };
    // console.log(angle, [layer_lv, id % countCap], countCap, id, position);

    return [position, angle];
  };

  const nodeMap = Object.values(
    Object.groupBy(nodes, (item) => item.layer)
  ).map((layer, layer_lv) => {
    const cap = layer.length;
    return {
      layer_ref: null,
      items: layer.map((node, index_) => {
        // const { position, ...rest } = item;
        const [position, angle] = calc_xy(node.layer, index_, cap);
        return {
          ...node,
          position,
          angle,
          isActive: layer_lv === 0 || layer_lv === 1,
        };
      }),
      layer_rotate: 0,
      layer_rotate_sub: null,
      // layer_rotate: tweened(0, { duration: 300, easing: cubicOut }),
    };
  });

  const onItemClicked = (layer_lv, id) => {
    const node = nodeMap[layer_lv].items.find((item) => item.id === id);
    currentLayerLv = node.layer + 1;
    if (nodeMap[layer_lv + 1] === undefined) return;
    nodeMap[layer_lv + 1].items.forEach((item) => {
      item.isActive = item.parent === id;
    });
    nodeMap[layer_lv].layer_rotate = node.angle * -1;
    // console.log(nodeMap[layer_lv])
    if (nodeMap[layer_lv].layer_rotate_sub) {
      nodeMap[layer_lv].layer_rotate_sub.set(node.angle * -1);
    }
    console.log(nodeMap[layer_lv]);

  };
  const onLayerCreated = (event, layer) => {
    layer.layer_ref = event.detail.instance;
    layer.layer_rotate_sub = tweened(0, { duration: 300, easing: cubicOut });
    layer.layer_rotate_sub.subscribe((v) => {
      layer.layer_ref.rotation = v;
    });
  };
</script>

<Application width={1400} height={1400} backgroundColor={0x1099bb}>
  <Container anchor={0.5} pivot={0.5}>
    {#each nodeMap as layer, layer_lv}
      <Container
        x={center.x}
        y={center.y}
        anchor={0.5}
        pivot={0.5}
        rotation={layer.layer_rotate}
        on:create={(e) => onLayerCreated(e, layer)}
      >
        {#if layer_lv !== 0}
          <!-- content here -->
          <Graphics
            zIndex={2}
            draw={(graphics) => {
              graphics.clear();
              // graphics.beginFill(0xffffff);
              graphics.lineStyle(7, 0x555555);
              graphics.drawCircle(0, 0, layer_lv * radius);
              graphics.endFill();
            }}
          />
        {/if}
        {#each layer.items as item}
          <!-- content here -->
          <Station
            outerRotate={layer.layer_rotate}
            position={item.position}
            icon={item.icon}
            label={item.name}
            isActive={item.isActive || item.layer <= currentLayerLv}
            onClick={() => onItemClicked(layer_lv, item.id)}
          />
        {/each}
      </Container>
    {/each}
  </Container>
</Application>
