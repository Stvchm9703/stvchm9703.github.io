<script lang="ts">
  import Block from "./layout.svelte";

  import {
    Table,
    TableHeader,
    TableHead,
    TableBody,
    TableRow,
    TableCell,
  } from "$lib/components/ui/table";

  const block = $props();
  const { id, order, layoutStyle, componentAttr, style, items, ...rest } =
    block;
  // const { componentType, layoutStyle } = block;

  import { cn } from "$lib/utils";
  import { headerIdResolver } from "./common";
    import BlockTable from "$lib/components/post-content-layout/block/table.svelte";
//   import { onMount } from "svelte";

  //   // id={headerIdResolver(componentType, id)}
  //   onMount(() => {
  //     // console.log(">>> Layout Block Items: ", items);
  //     if (layoutStyle === "Table") {
  //       console.log(">>> Layout Block ID: ", id, " Order: ", order);
  //       console.log(">>> Layout Block : ", block);
  //     }
  //   });
</script>

{#if layoutStyle === "Table"}
  <BlockTable {...block} />
{:else}
  <div
    id={headerIdResolver(`layout_${layoutStyle}`, String(id || order) || "")}
    data-layout-style={layoutStyle}
    class={cn([
      {
        "flex flex-col lg:flex-row": layoutStyle === "Row",
        "block flex-1": layoutStyle === "Column",
      },
    ])}
  >
    {#each items as child_blk}
        <Block {...child_blk} />
    {/each}
  </div>
{/if}
