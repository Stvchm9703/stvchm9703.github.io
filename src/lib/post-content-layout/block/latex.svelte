<script lang="ts">
    import type { ContentBlock } from "$generateor/content_block";
    import { onMount } from "svelte";
    import { cn } from "$lib/utils";
    import {
        resolveMarks,
        pathResolver,
        headerIdResolver,
        resolveStyle,
    } from "./common";
    const {
        id,
        fields,
        componentAttr,
        style: content_style,
        ...rest
    }: ContentBlock = $props();

    const { text, processor } = componentAttr;
    const resolveYoutubeEmbed = (link: string) => {
        const v = link.split("?v=");
        return `https://www.youtube.com/embed/${v[1]}`;
    };

    let mermaid = $state.raw(null);
    let mermaid_rend = $state.raw(null);
    onMount(async () => {
        if (processor === "Mermaid") {
            mermaid = (await import("mermaid")).default;
            console.log(mermaid);
            console.log(mermaid_rend);

            await mermaid.init();
            const result = await mermaid.render(
                headerIdResolver(processor, id),
                text,
            );
            console.log(result);
            mermaid_rend = result.svg;
            console.log(mermaid_rend);
        }
    });
</script>

{#if processor === "Youtube"}
    <iframe
        id={headerIdResolver(processor, id)}
        class={cn(["w-full aspect-video", resolveStyle(content_style)])}
        src={resolveYoutubeEmbed(text)}
        frameBorder="0"
        allowFullScreen
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    />
{:else if processor === "GoogleMaps"}
    <iframe
        id={headerIdResolver(processor, id)}
        class={cn(["w-full aspect-video", resolveStyle(content_style)])}
        src={text}
    >
        Google Map content, unresovleds
    </iframe>
{:else if processor === "Mermaid"}
    <figure
        class={cn([
            "w-[70%] mx-auto aspect-video flex items-center bg-slate-100 justify-center p-6",
            resolveStyle(content_style),
        ])}
    >
        {@html mermaid_rend}
    </figure>
{:else if processor === "Graphviz"}{/if}
