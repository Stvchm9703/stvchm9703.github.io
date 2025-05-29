<script lang="ts">
    import Block from "./layout.svelte";
    const { id, layoutStyle, componentAttr, style, items } = $props();
    // const { componentType, layoutStyle } = block;

    import { cn } from "$lib/utils";
    import {
        resolveMarks,
        pathResolver,
        headerIdResolver,
        resolveStyle,
    } from "./common";
    // id={headerIdResolver(componentType, id)}
</script>

<div
    id={headerIdResolver("layout-" + layoutStyle, id || "")}
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
