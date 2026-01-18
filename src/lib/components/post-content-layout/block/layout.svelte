<script lang="ts">
    // import Self from "./layout.svelte";
    const block = $props();
    // const { componentType, layoutStyle } = block.componentAttr;

    const componentType = block.componentAttr?.componentType || "Unkown";
    const layoutStyle = block.componentAttr?.layoutStyle || "";
    
    const items = block.componentAttr?.items || [];

    import { cn } from "$lib/utils";
    import { headerIdResolver } from "./common";
    // id={headerIdResolver(componentType, id)}

    // CMS - content block render Component
    import FlexBlock from "$lib/components/post-content-layout/block/flex_block.svelte";
    import BlockText from "$lib/components/post-content-layout/block/text.svelte";
    import BlockCode from "$lib/components/post-content-layout/block/code.svelte";
    import BlockLatex from "$lib/components/post-content-layout/block/latex.svelte";
    import BlockTable from "$lib/components/post-content-layout/block/table.svelte";
    import BlockBookmark from "$lib/components/post-content-layout/block/bookmark.svelte";
    import BlockLink from "$lib/components/post-content-layout/block/link.svelte";
    import BlockFile from "$lib/components/post-content-layout/block/file.svelte";
    import BlockJupyter from "$lib/components/post-content-layout/block/custom/jupyter.svelte";
    // import BlockLayout from "$lib/components/post-content-layout/block/layout.svelte";

    // console.log();
</script>

{#if componentType === "Layout"}
    <div
        id={headerIdResolver(componentType + "_" + layoutStyle, block.id || "")}
        class={cn([
            {
                "flex flex-col lg:flex-row": layoutStyle === "Row",
                "block flex-1": layoutStyle === "Column",
            },
        ])}
    >
        {#each items as child_blk, childIDX}
            <FlexBlock {...child_blk} />
        {/each}
    </div>
{:else if componentType === "Text" && block.componentAttr["style"] === "Code"}
    <BlockCode {...block} />
{:else if componentType === "JupyterComponent"}
    <BlockJupyter {...block} />
{:else if componentType === "Text"}
    <BlockText {...block} />
{:else if componentType === "Table"}
    <BlockTable {...block} />
{:else if componentType === "Bookmark"}
    <BlockBookmark {...block} />
{:else if componentType === "File"}
    <BlockFile {...block} />
{:else if componentType === "Link"}
    <BlockLink {...block} />
{:else if componentType === "Latex"}
    <BlockLatex {...block} />
{:else if componentType === "CustomComponent"}
    <!-- <span> not yet implement : {componentType} </span> -->
{:else}
    <!-- <span> not yet implement : {componentType} </span> -->
{/if}
