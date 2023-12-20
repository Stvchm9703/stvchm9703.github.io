<script>
  import { nodes } from "./data.js";
  import Station from "./station.svelte";
</script>

<div class="overflow-y-scroll max-h-full block">
  <div class="flex flex-col">
    {#each nodes as node}
      <Station label={node.name} icon={node.icon} />
    {/each}
  </div>
</div>
