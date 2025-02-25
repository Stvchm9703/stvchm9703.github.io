<script>
  import { onMount } from "svelte";
  import { Picture } from "svelte-lazy-loader";

  // export let alt = "";
  // export let height = ""; // needed to reduce CLS
  // export let width = ""; // needed to reduce CLS
  // export let srcList = [];
  // export const imagePath = "";
  // export let forceLoad = false;

  const { alt, height, width, srcList, imagePath, forceLoad, ...props } =
    $props();
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

  // $: remapSrcList = (
  //   srcList && Array.isArray(srcList)
  //     ? srcList.sort((a, b) => a.width - b.width)
  //     : srcList
  //       ? [srcList]
  //       : []
  // ).map((elm) => ({ ...elm, media: `(min-width: ${elm.width + 1}px)` }));

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
  >
  </div>  
{/if}
