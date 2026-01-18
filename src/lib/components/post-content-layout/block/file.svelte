<script lang="ts">
    import type { ContentBlock } from "$generateor/content_block";
    import { cn } from "$lib/utils";
    import { headerIdResolver, resolveStyle } from "./common";
    import {
        // initLightGallery,
        // lightgallery,
        openGallery,
    } from "$lib/stores/lightgallery.svelte";

    const {
        postTitle,
        id,
        fields,
        componentAttr,
        // style: content_style,
        ...rest
    }: ContentBlock = $props();

    const { type: fileType, style, name: fileName, fileUrl } = componentAttr;
    const baseClass = ["w-full", "my-2", "max-h-240"];
    const mediaClass = [
        "rounded-md",
        "bg-slate-100",
        "w-full",
        "aspect-16/9",
        "object-contain",
    ];

    const elemId = headerIdResolver(fileType, id);
</script>

<template>
    {JSON.stringify(componentAttr)}
</template>

{#if fileType === "Image"}
    <figure
        id={elemId}
        class={cn([baseClass, resolveStyle(style)])}
        onclick={() => openGallery(fileUrl)}
    >
        <figcaption></figcaption>
        <picture>
            <img
                class={cn(mediaClass)}
                src={`/blog/assets/${fileUrl}`}
                alt="{fileName} - {postTitle}"
                aria-label={fileName}
            />
        </picture>
    </figure>
{:else if fileType === "Video"}
    <figure id={elemId} class={cn([baseClass, resolveStyle(style)])}>
        <figcaption></figcaption>
        <video class={cn(mediaClass)} src={`/blog/assets/${fileUrl}`}>
            <track kind="captions" />
        </video>
    </figure>
{:else if fileType === "File"}
    <a href={`/blog/assets/${fileUrl}`} download>
        {fileName}
    </a>
{/if}

