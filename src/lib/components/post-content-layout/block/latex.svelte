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
        id: blockId,
        fields,
        componentAttr,
        style: content_style,
        ...rest
    }: ContentBlock = $props();

    const { text, processor } = componentAttr;
    const id = headerIdResolver(processor, blockId);
    const baseComponetClass = ["mx-auto", "aspect-video", "my-2"];

    /// google map

    function valueFromZoom(z: number): number {
        const A = 451272000;
        const B = 0.508;
        return A * Math.pow(B, z);
    }
    function convertGoogleMapsUrlToEmbed(url: string): string | null {
        // Extract latitude, longitude, and zoom from the URL using regex
        const coordMatch = url.match(
            /@([-+]?\d*\.\d+|[-+]?\d+),([-+]?\d*\.\d+|[-+]?\d+),(\d+)z/,
        );
        if (!coordMatch) {
            return null;
        }

        const latitude = parseFloat(coordMatch[1]);
        const longitude = parseFloat(coordMatch[2]);
        const zoom = parseFloat(coordMatch[3]);
        // Extract place name from the URL
        const placeMatch = url.match(/\/place\/([^\/@]+)/);
        let placeName = "";
        if (placeMatch) {
            placeName = decodeURIComponent(placeMatch[1].replace(/\+/g, " "));
        }
        // console.log(placeMatch, coordMatch, coordMatch[3]);

        // Construct the pb parameter string for embed URL
        // Note: This is a simplified static pb string with dynamic lat/lng and place name inserted
        const pbString = `!1m18!1m12!1m3!1d${valueFromZoom(zoom)}!2d${longitude}!3d${latitude}!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s${encodeURIComponent(placeName)}!2s${encodeURIComponent(placeName)}!5e0!3m2!1sen!2shk!4v1745579825168!5m2!1sen!2shk`;

        const embedUrl = `https://www.google.com/maps/embed?pb=${pbString}`;
        return embedUrl;
    }

    /// youtube
    const resolveYoutubeEmbed = (link: string) => {
        const v = link.split("?v=");
        return `https://www.youtube.com/embed/${v[1]}`;
    };

    // meriad implement
    let mermaid_rend = $state.raw(null);
    onMount(async () => {
        if (processor === "Mermaid") {
            const mermaid = (await import("mermaid")).default;
            await mermaid.init();
            const result = await mermaid.render(id, text);
            mermaid_rend = result.svg;
            // console.log(mermaid_rend);
        }
    });

    // Graphviz / dot-lang
    let graphviz_rend: string | null = $state.raw(null);
    onMount(async () => {
        if (processor === "Graphviz") {
            const viz = (await import("@viz-js/viz")).instance;
            const render = await viz();
            graphviz_rend = render.renderSVGElement(text);
        }
    });
</script>

{#if processor === "Youtube"}
    <iframe
        {id}
        class={cn([baseComponetClass, "w-full", resolveStyle(content_style)])}
        src={resolveYoutubeEmbed(text)}
        frameBorder="0"
        allowFullScreen
        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
        title="yt-link"
        loading="lazy"
    ></iframe>
{:else if processor === "GoogleMaps"}
    <iframe
        src={convertGoogleMapsUrlToEmbed(text)}
        class={cn([baseComponetClass, "w-full", resolveStyle(content_style)])}
        title="google-map"
        allowfullscreen
        loading="lazy"
        referrerpolicy="no-referrer-when-downgrade"
    >
        Google Map content, unresovleds
    </iframe>
{:else if processor === "Mermaid"}
    <figure
        id={`container-${id}`}
        class={cn([
            baseComponetClass,
            "flex items-center bg-slate-100 justify-center p-6",
            resolveStyle(content_style),
        ])}
    >
        {@html mermaid_rend}
    </figure>
{:else if processor === "Graphviz"}
    <figure
        id={`container-${id}`}
        class={cn([
            baseComponetClass,
            "flex items-center bg-slate-100 justify-center p-6",
            resolveStyle(content_style),
        ])}
    >
        {@html graphviz_rend?.outerHTML}
    </figure>
{/if}
