<script>
  import "@carbon/charts/styles.css";
  import "./styles.css";
  import MainContainer from "./containers/main.svelte";
  import ModalView from "./containers/modal.svelte";
  import Edge from "./edge.svelte";
  import Card from "./card.svelte";
  import * as d3 from "d3";
  // import * as d3Zoom from "d3-zoom";
  import { onMount } from "svelte";
  import { nodes } from "../data.js";
  import sqids from "sqids";

  import Fuse from "fuse.js";
  import DataTable from "./containers/data-table.svelte";
  let svgContainer = $state();

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

  let repoData = [],
    fuseSearchEng = null;

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
    .map((node, nodeIndex) =>
      node.parent !== null
        ? {
            id: nodeIndex,
            source: instanceNodes.find(
              (instanceNode) => instanceNode.id === node.parent
            ),
            target: node,
            silberingCount: node.silberingCount,
            silberingOrder: node.silberingOrder,
          }
        : null
    )
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

  let focusOnNodes = $state([]),
    focusOnEdges = $state([]);
  let inViewNode = $state(null);
  let inViewDetail = $state(null);
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
    inViewNode = instanceNodes.find((elm) => elm.id === cardId);
    // console.log(inViewNode.name);
    if (fuseSearchEng && inViewNode) {
      let searchResult = fuseSearchEng.search(inViewNode.name);

      const result = searchResult.filter((elm) => elm.score >= 0.5);

      // console.log(result);
      inViewDetail = result.map((elm) => elm.item);
    }
    // inViewDetail = import(`$assets/data/${inViewNode.name}.md`);
  };

  const onCardActionClick = (cardId) => {
    // console.log("on-card-action ", cardId);
    const inWatchNode = instanceNodes.find((elm) => elm.id === cardId);
    let focusedIds = [
      ...inWatchNode?.parentChain,
      inWatchNode?.id,
      ...inWatchNode?.children,
    ];

    let parentAlignment = [...inWatchNode?.parentChain, inWatchNode?.id];
    focusOnNodes = instanceNodes.filter((node) => focusedIds.includes(node.id));
    focusOnNodes = focusOnNodes.map((node) => {
      let layer = node.parentChain.length;
      let { silberingOrder, parent } = node;
      let position =
        parent !== null
          ? {
              x: offsetWidth * (layer + 1) + (offsetWidth + unitWidth) * layer,
              y: parentAlignment.includes(node.id)
                ? offsetHeight
                : offsetHeight * (silberingOrder + 1) +
                  unitHeight * silberingOrder,
            }
          : node.position;

      return {
        ...node,
        position,
      };
    });

    focusOnEdges = focusOnNodes
      .map((node, nodeIndex) =>
        node.parent !== null
          ? {
              id: nodeIndex,
              source: focusOnNodes.find(
                (instanceNode) => instanceNode.id === node.parent
              ),
              target: node,
              silberingCount: parentAlignment.includes(node.id)
                ? 1
                : focusOnNodes.filter((elm) => elm.layer === node.layer).length,
              silberingOrder: parentAlignment.includes(node.id)
                ? 0
                : focusOnNodes
                    .filter((elm) => elm.layer === node.layer)
                    .findIndex((elm) => elm.id === node.id),
            }
          : null
      )
      .filter((edge) => edge !== null);
  };

  const reset = () => {
    focusOnNodes = [];
    focusOnEdges = [];
  };

  onMount(async () => {
    init();
    let importData = await import(`$assets/content/full-repo-data.json`);
    repoData = importData.default.map((elm) => ({
      ...elm,
      languages_key: Object.keys(elm.languages),
    }));
    // repoData = importData;
    fuseSearchEng = new Fuse(repoData, {
      keys: ["tags", "languages", "description"],
      includeScore: true,
      distance: 5,
      shouldSort: true,
    });
  });
</script>

<MainContainer>
  <div class="absolute top-2 right-2 bg-gray-100">
    {#if focusOnNodes.length !== 0}
      <button class="cds--cc--card-node--button" onclick={reset}>
        Reset
      </button>
    {/if}
  </div>
  <svg bind:this={svgContainer} class="layout-svg-wrapper !overflow-hidden">
    <g class="layer--edge">
      {#each focusOnNodes.length === 0 ? edges : focusOnEdges as edge}
        <Edge
          assignId={sqid.encode([edge.id, edge.source.id, edge.target.id])}
          source={edgeSourcePosition(edge)}
          target={edge.target.position}
          middleAxisAdjust={edgeMiddleAxisAdjust(edge)}
          transform="translate({unitWidth / 2}, {unitHeight / 2})"
          color={edge.target.color}
          variant={edge.target.layer - edge.source.layer > 1 ? "dash-md" : null}
        />
      {/each}
    </g>
    <g class="layer--card">
      {#each focusOnNodes.length === 0 ? instanceNodes : focusOnNodes as node}
        <Card
          id={node.id}
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
      {/each}
    </g>
  </svg>
  {#snippet modal()}
    <ModalView
      isShow={inViewNode !== null}
      onCloseClick={() => (inViewNode = null)}
    >
      {#if inViewNode !== null}
        <div class="px-3">
          <h1 class="text-xl">{inViewNode.name}</h1>
          <hr />
          <DataTable data={inViewDetail} />
        </div>
      {/if}
    </ModalView>
  {/snippet}
</MainContainer>
