<script>
  import * as PIXI from "pixi.js";
  import {
    Text,
    Sprite,
    Container,
    Graphics,
    onTick,
    getContainer,
  } from "svelte-pixi";
  import { tweened } from "svelte/motion";
  import { cubicOut } from "svelte/easing";

  export let icon = null;
  // "https://s3-us-west-2.amazonaws.com/s.cdpn.io/693612/IaUrttj.png";
  import { DropShadowFilter } from "@pixi/filter-drop-shadow";

  export let position = { x: 0, y: 0, z: 1 };
  // export let scale = 0.5;
  export let label = "Hello World";
  export let isActive = false;

  export let outerRotate = 0;
  let isHover = false,
    isClick = false;
  const shadowFilter = new DropShadowFilter({
    offset: { x: 0, y: 2 },
  });
  // const {container} = getContainer();
  // current.pivot.x = current.width / 2;
  // current.pivot.y = current.height / 2;
  let instance;
  // console.log(instance);

  // console.log(current);

  const scale = tweened(0.7, {
    duration: 300,
    easing: cubicOut,
  });
  const shadowFilterAlpha = tweened(0.5, {
    duration: 300,
    easing: cubicOut,
  });
  const outerRotateTween = tweened(outerRotate, {
    duration: 750,
    easing: cubicOut,
  });

  // onTick(() => {
  //   // current.visible = isActive;
  // });

  $: instance && (instance.filters = [shadowFilter]);
  $: {
    $outerRotateTween =  outerRotate * -1;

    if (isHover || isClick) {
      $scale = 0.75;
      $shadowFilterAlpha = 1;
    } else {
      $scale = 0.7;
      $shadowFilterAlpha = 0.5;
    }
    shadowFilter.alpha = $shadowFilterAlpha;  
  }

  export let onClick = (event) => {};
  const onButtonDown = (event) => {
    isClick = true;
    onClick(event);
  };
  const onButtonUp = (event) => (isClick = false);
  const onButtonOver = (event) => (isHover = true);
  const onButtonOut = (event) => (isHover = false);

  const iconTexture = icon ? PIXI.Texture.from(icon) : null;
</script>

<Container
  bind:instance
  x={position.x}
  y={position.y}
  zIndex={position.z}
  anchor={0.5}
  interactive
  buttonMode
  scale={$scale}
  rotation={$outerRotateTween}
  hitArea={new PIXI.Circle(0, 0, 100)}
  on:pointerdown={onButtonDown}
  on:pointerup={onButtonUp}
  on:pointerupoutside={onButtonUp}
  on:pointerover={onButtonOver}
  on:pointerout={onButtonOut}
>
  <!-- based -->
  <Graphics
    x={0}
    y={0}
    draw={(graphics) => {
      graphics.clear();
      graphics.beginFill(0xffffff);
      graphics.lineStyle(7, 0x000000);
      graphics.drawCircle(0, 0, 25);
      graphics.endFill();
    }}
  />

  {#if iconTexture !== null}
    <Sprite
      x={0}
      y={-40}
      width={50}
      height={50}
      anchor={0.5}
      texture={iconTexture}
      skew={{ y: 0.25 }}
      style={{ fill: 0xffffff }}
    />
  {/if}
  <Text
    x={0}
    y={45}
    anchor={0.5}
    text={label}
    style={{
      fill: 0xffffff,
      stroke: 0x000000,
      strokeThickness: 3,
    }}
  />
</Container>
