<script lang="ts">
  import type { ContentBlock, FileComponentAttr } from "$generateor/content_block";
  import { cn } from "$lib/utils";
  import { headerIdResolver, resolveStyle } from "./common";
  import {
    // initLightGallery,
    // lightgallery,
    openGallery,
  } from "$lib/stores/lightgallery.svelte";

  const {
    id,
    fields,
    componentAttr,
    // style: content_style,
    ...rest
  }: ContentBlock = $props();

  // Narrow to FileComponentAttr — this component is only mounted for File blocks
  const fileAttr = $derived(
    componentAttr.componentType === "File"
      ? (componentAttr as FileComponentAttr)
      : null
  );

  const fileType = $derived(fileAttr?.type);
  const fileStyle = $derived(fileAttr?.style);
  const fileName = $derived(fileAttr?.name ?? "");
  const fileUrl = $derived(fileAttr?.fileUrl ?? "");

  const baseClass = ["w-full", "my-2", "max-h-240"];
  const mediaClass = [
    "rounded-md",
    "bg-slate-100",
    "w-full",
    "aspect-16/9",
    "object-contain",
  ];

  const elemId = $derived(headerIdResolver(fileType ?? "", id));
</script>

<template>
  {JSON.stringify(componentAttr)}
</template>

{#if fileType === "Image"}
  <figure
    id={elemId}
    class={cn([baseClass, resolveStyle(fileStyle)])}
    onclick={() => openGallery(fileUrl)}
  >
    <figcaption></figcaption>
    <picture>
      <img
        class={cn(mediaClass)}
        src={`/blog/assets/${fileUrl}`}
        alt="{fileName}"
        aria-label={fileName}
      />
    </picture>
  </figure>
{:else if fileType === "Video"}
  <figure id={elemId} class={cn([baseClass, resolveStyle(fileStyle)])}>
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
