<script lang="ts" module>
  export interface ImageSource {
    src: string;
    width: number;
    format: string;
    media?: string;
  }

  export interface ImageProps {
    alt?: string;
    height?: number | string;
    width?: number | string;
    srcList?: ImageSource[] | ImageSource;
    imagePath?: string;
    forceLoad?: boolean;
    class?: string;
  }
</script>

<script lang="ts">
  // import { onMount } from "svelte";
  import Picture from "./lazy-picture.svelte";

  const { alt, height, width, srcList, imagePath, forceLoad, ...props } : ImageProps = $props();
  // export let srcList = [];
  const baseSet = $derived(
    srcList && Array.isArray(srcList)
      ? (srcList.filter((elm) => elm.format == "webp").length > 0
          ? srcList.filter((elm) => elm.format == "webp")
          : srcList
        ).sort((a, b) => a.width - b.width)[0]
      : srcList
        ? srcList
        : null
  );

  const remapSrcList = $derived(
    (srcList && Array.isArray(srcList)
      ? srcList.sort((a, b) => a.width - b.width)
      : srcList
        ? [srcList]
        : []
    ).map((elm) => ({ ...elm, media: `(min-width: ${elm.width + 1}px)` }))
  );
</script>

<svelte:head>
  {#if baseSet && forceLoad}
    <link
      rel="preload"
      as="image"
      href={baseSet.src}
      imagesrcset={remapSrcList
        .map((elm) => `${elm.src} ${elm.width}w`)
        .join(", ")}
      imagesizes="100vw"
      data-srcset={remapSrcList.map((elm) => elm.src).join(", ")}
      data-image-prefetch-ref={alt}
    />
  {/if}
</svelte:head>
{#if baseSet}
  <Picture
    classes="children:h-full children:w-full children:object-cover {props.class}"
    loading={forceLoad ? "eager" : "lazy"}
    {alt}
    {height}
    {width}
  >
    {#each remapSrcList as { src, format, media }, index}
      <source data-srcset={src} {media} type="image/{format}" />
    {/each}
  </Picture>
{:else}
  <div
    class="bg-dark-100 children:h-full children:w-full children:object-cover {props.class}"
  ></div>
{/if}
