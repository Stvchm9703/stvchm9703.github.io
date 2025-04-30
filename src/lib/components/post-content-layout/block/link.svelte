<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import type { ContentBlock } from "$generateor/content_block";
    import Tag from "$lib/components/post-content-layout/tag/tag.svelte";
    import { resolveMarks, resolveStyle, headerIdResolver } from "./common";

    let { class: className, id, componentAttr, componentType } = $props();
</script>

{#if componentAttr["cardStyle"] === "Card"}
    <Card.Root
        id={headerIdResolver(componentType, id)}
        isElement="a"
        stComponent={componentType}
        href={componentAttr.href}
        class={[resolveStyle(componentAttr["style"]), "my-2", className]}
        target="_blank"
    >
        <Card.Content class={["py-4", "flex"]}>
            <img alt={componentAttr.title} class="w-8 h-8 rounded-full mr-4" />
            <div class="flex-grow flex-col">
                <Card.Title class="text-sm font-semibold"
                    >{componentAttr.title && componentAttr.title !== ""
                        ? componentAttr.title
                        : componentAttr.href}</Card.Title
                >
                <Card.Description class="flex flex-col ">
                    <p class="text-sm font-normal">
                        {componentAttr.description}
                    </p>
                    <div class="flex flex-row gap-4">
                        <span class="text-xs font-normal">
                            {componentAttr.date}
                        </span>
                        {#each componentAttr.tags as tag}
                            <Tag {...tag} />
                        {/each}
                    </div>
                </Card.Description>
            </div>
        </Card.Content>
    </Card.Root>
{/if}
