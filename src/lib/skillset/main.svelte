<script>
  // import "@carbon/charts/styles.css";
  // import "./styles.css";
  // import MainContainer from "./containers/main.svelte";
  // import ModpalView from "./containers/modal.svelte";
  // import Edge from "./edge.svelte";
  import Card from "./card.svelte";
  // import * as d3 from "d3";
  // import * as d3Zoom from "d3-zoom";
  import { onMount } from "svelte";
  import { nodes } from "./data.js";
  import sqids from "sqids";

  // import Fuse from "fuse.js";
  // import DataTable from "./containers/data-table.svelte";
  // let svgContainer = $state();

  // import "@selemondev/svelte-marquee/dist/style.css";
  import "./style.css";
  import { Marquee } from "@selemondev/svelte-marquee";
</script>

<div class="w-full h-full">
  <!-- filter -->

  <div class="flex flex-wrap gap-8">
    <Marquee fade={true} pauseOnHover={true}>
      {#each nodes.slice(0, nodes.length / 3) as skillItem}
        <Card
          iconSrc={skillItem.icon}
          colorCode={skillItem.color}
          title={skillItem.name}
        />
      {/each}
    </Marquee>
    <Marquee fade={true} reverse={true} pauseOnHover={true}>
      {#each nodes.slice(nodes.length / 3, (nodes.length / 3) * 2) as skillItem}
        <Card
          iconSrc={skillItem.icon}
          colorCode={skillItem.color}
          title={skillItem.name}
        />
      {/each}
    </Marquee>
    <Marquee fade={true} pauseOnHover={true}>
      {#each nodes.slice((nodes.length / 3) * 2) as skillItem}
        <Card
          iconSrc={skillItem.icon}
          colorCode={skillItem.color}
          title={skillItem.name}
        />
      {/each}
    </Marquee>
  </div>
</div>
