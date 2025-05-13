<script lang="ts">
    import Self from "./layout.svelte";
    const block = $props();
    import { cn } from "$lib/utils";
    import {
        resolveMarks,
        pathResolver,
        headerIdResolver,
        resolveStyle,
    } from "./common";
    // id={headerIdResolver(componentType, id)}

    // CMS - content block render Component
    import BlockText from "$lib/components/post-content-layout/block/text.svelte";
    import BlockCode from "$lib/components/post-content-layout/block/code.svelte";
    import BlockLatex from "$lib/components/post-content-layout/block/latex.svelte";
    import BlockTable from "$lib/components/post-content-layout/block/table.svelte";
    import BlockBookmark from "$lib/components/post-content-layout/block/bookmark.svelte";
    import BlockLink from "$lib/components/post-content-layout/block/link.svelte";
    import BlockFile from "$lib/components/post-content-layout/block/file.svelte";
    import BlockJupyter from "$lib/components/post-content-layout/block/custom/jupyter.svelte";
    // import BlockLayout from "$lib/components/post-content-layout/block/layout.svelte";
</script>

{#if block.componentType.includes("Layout")}
    <div
        id={headerIdResolver(block.componentType, block.id)}
        class={cn([
            {
                "flex flex-col lg:flex-row":
                    block.componentType === "LayoutRow",
                "block flex-1": block.componentType === "LayoutColumn",
            },
        ])}
    >
        {#each block.children as child_blk}
            <Self {...child_blk} />
        {/each}
    </div>
{:else if block.componentType === "Text" && block.componentAttr["style"] === "Code"}
    <BlockCode {...block} />
{:else if block.componentType === "JupyterComponent"}
    <BlockJupyter {...block} />
{:else if block.componentType === "Text"}
    <BlockText {...block} />
{:else if block.componentType === "Table"}
    <BlockTable {...block} />
{:else if block.componentType === "Bookmark"}
    <BlockBookmark {...block} />
{:else if block.componentType === "File"}
    <BlockFile {...block} />
{:else if block.componentType === "Link"}
    <BlockLink {...block} />
{:else if block.componentType === "Latex"}
    <BlockLatex {...block} />
{:else if block.componentType === "Layout" || block.componentType === "TableOfContents" || block.componentType === "Relation"}
    <!-- <span> not yet implement : {block.componentType} </span> -->
{:else if block.componentType === "CustomComponent"}
    <!-- <span> not yet implement : {block.componentType} </span> -->
{:else}
    <!-- <span> not yet implement : {block.componentType} </span> -->
{/if}
