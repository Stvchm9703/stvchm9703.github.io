<script>
  import "@carbon/charts/styles.css";
  import "./styles.css";
  import MainContainer from "./containers/main.svelte";
  import ModalView from "./containers/modal.svelte";
  import Edge from "./edge.svelte";
  import Card from "./card.svelte";
  import * as d3 from "d3";
  import * as d3Zoom from "d3-zoom";
  import { onMount } from "svelte";
  import { nodes } from "../data.js";
  import sqids from "sqids";
  let svgContainer;

  let unitWidth = 200,
    unitHeight = 64,
    offsetWidth = 120,
    offsetHeight = 80;
  // const groupBy = (values, keyFinder) => {
  //   return values.reduce((a, b) => {
  //     const key = typeof keyFinder === "function" ? keyFinder(b) : b[keyFinder];
  //     if (!a[key]) {
  //       a[key] = [b];
  //     } else {
  //       a[key] = [...a[key], b];
  //     }
  //     return a;
  //   }, {});
  // };

  const sqid = new sqids({
    minLength: 4,
    alphabet: "FxnXM1kBN6cuhsAvjW3Co7l2RePyY8DwaU04Tzt9fHQrqSVKdpimLGIJOgb5ZE",
  });

  const parentChain = (node) => {
    const parent = nodes.find(
      (instanceNode) => instanceNode.id === node.parent
    );
    if (parent) {
      return [parent.id, ...parentChain(parent)];
    }
    return [];
  };

  // let layers = Object.keys(groupBy(nodes, "layer"));
  let instanceNodes = nodes.map((item, nodeIndex) => {
    const { layer } = item;

    const silberingNodes = nodes
      .filter((elm) => elm.parent === item.parent)
      .sort((a, b) => a.id - b.id)
      .map((elm) => elm.id);
    const parentChainList = parentChain(item).sort((a, b) => a - b);
    // const layer = parentChainList.length;
    const layerNodeIndex = nodes
      .filter((node) => node.layer === item.layer)
      .findIndex((node) => node.id === item.id);
    return {
      ...item,
      children: nodes
        .filter((node) => node.parent === item.id)
        .map((node) => node.id),
      silberingCount: silberingNodes.length,
      silberingOrder: silberingNodes.findIndex((elm) => elm === item.id),
      parentChain: parentChainList,
      // layer,
      position: {
        x: offsetWidth * (layer + 1) + (offsetWidth + unitWidth) * layer,
        y: offsetHeight * (layerNodeIndex + 1) + unitHeight * layerNodeIndex,
      },
    };
  });
  // instanceNodes = instanceNodes.map((elm) => {
  //   const { layer } = elm;
  //   const nodeIndex = instanceNodes
  //     .filter((node) => node.layer === elm.layer)
  //     .findIndex((node) => node.id === elm.id);
  //   return {
  //     ...elm,
  //     position: {
  //       x: offsetWidth * (layer + 1) + (offsetWidth + unitWidth) * layer,
  //       y: offsetHeight * (nodeIndex + 1) + unitHeight * nodeIndex,
  //     },
  //   };
  // });

  // let maxItems = Math.max(...instanceNodes1.map((layer) => layer.length));

  // const instanceNodes = instanceNodes1.reduce((acc, layer) => {
  //   return [...acc, ...layer];
  // }, []);

  const edges = instanceNodes
    .map((node, nodeIndex) => {
      return node.parent !== null
        ? {
            id: nodeIndex,
            source: instanceNodes.find(
              (instanceNode) => instanceNode.id === node.parent
            ),
            target: node,
            silberingCount: node.silberingCount,
            silberingOrder: node.silberingOrder,
          }
        : null;
    })
    .filter((edge) => edge !== null);

  const edgeSourcePosition = (edge) => {
    let x = edge.source.position.x; /** base position */

    let y =
      edge.source.position.y /** base position */ +
      (edge.silberingCount > 1
        ? (edge.silberingOrder - edge.silberingCount / 2) * 8
        : 0) /** silbering */ +
      0;
    return { x, y };
  };
  const edgeMiddleAxisAdjust = (edge) => {
    if (edge.target.layer - edge.source.layer === 0) return 0;
    return (
      (edge.silberingCount > 1
        ? (edge.silberingOrder - edge.silberingCount / 2) * 8
        : 0) /** silbering */ +
      (edge.source.silberingCount > 1
        ? (edge.source.silberingOrder - edge.source.silberingCount / 2) * 40
        : 0) /** parent-silbering */ +
      0
      // (edge.target.layer - edge.source.layer > 1 ? -105 : 0) /** over-layer */
    );
  };

  let focusOnNodes = [];
  let inViewNode = null;
  let inViewDetail = null;
  // d3 module
  const init = () => {
    const svgSelection = d3.select(svgContainer);
    const allRenderLayers = svgSelection.selectAll(
      "svg.layout-svg-wrapper > g"
    );
    const zoom = d3
      .zoom()
      .scaleExtent([0.5, 32])
      .on("zoom", ({ transform }) => {
        allRenderLayers.attr("style", `transform: ${transform.toString()};`);
        allRenderLayers.attr("transform", transform);
      });
    svgSelection.call(zoom);
  };

  const onCardClick = (cardId) => {
    console.log("on-card-click, ", cardId);
    inViewNode = instanceNodes.find((elm) => elm.id === cardId);
    inViewDetail = import(`$assets/data/${inViewNode.name}.md`);
  };
  const onCardActionClick = (cardId) => {
    // console.log("on-card-action ", cardId);
    const inWatchNode = instanceNodes.find((elm) => elm.id === cardId);
    focusOnNodes = [
      ...inWatchNode?.parentChain,
      inWatchNode?.id,
      ...inWatchNode?.children,
    ];

    // console.log(focusOnNodes);
  };

  onMount(() => {
    init();

    // console.table(instanceNodes);
    // console.table(edges);
    // console.table(instanceNodes , ["id", "name",]);
  });
</script>

<MainContainer>
  <svg bind:this={svgContainer} class="layout-svg-wrapper !overflow-hidden">
    <g class="layer--edge">
      {#each edges as edge}
        {#if focusOnNodes.length === 0 || focusOnNodes.includes(edge.target.id)}
          <Edge
            assignId={sqid.encode([edge.id, edge.source.id, edge.target.id])}
            source={edgeSourcePosition(edge)}
            target={edge.target.position}
            middleAxisAdjust={edgeMiddleAxisAdjust(edge)}
            transform="translate({unitWidth / 2}, {unitHeight / 2})"
            color={edge.target.color}
            variant={edge.target.layer - edge.source.layer > 1
              ? "dash-md"
              : null}
          />
        {/if}
      {/each}
    </g>
    <g class="layer--card">
      {#each instanceNodes as node}
        {#if focusOnNodes.length === 0 || focusOnNodes.includes(node.id)}
          <Card
            pos={node.position}
            title={node.name}
            stacked={node.children.length > 0}
            actions={node.children.length > 0}
            color={node.color}
            active={focusOnNodes.length === 0 || focusOnNodes.includes(node.id)}
            on:click={() => onCardClick(node.id)}
            onActionClick={() => onCardActionClick(node.id)}
            iconSrc={node.icon}
          />
        {/if}
      {/each}
    </g>
  </svg>
  <ModalView
    slot="modal"
    isShow={inViewNode !== null}
    onCloseClick={() => (inViewNode = null)}
  >
    {#if inViewNode !== null}
      <div class="px-3">
        <h1>{inViewNode.name}</h1>
        <hr />

      </div>
    {/if}
  </ModalView>
</MainContainer>
