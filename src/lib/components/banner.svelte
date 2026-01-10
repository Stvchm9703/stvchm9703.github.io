<script lang="ts" module>
import type { Snippet } from "svelte";

  export interface Props {
    imageSrc?: string[];
    imagePath?: string;
    id?: string;
    isFullScreen?: boolean;
    containerClass?: string;
    onIntersect?: (event: any) => void;
    background: Snippet;
    children?: Snippet;
  }
</script>

<!-- @migration-task Error while migrating Svelte code: $$props is used together with named props in a way that cannot be automatically migrated. -->
<script lang="ts">
  import { cn } from "$lib/utils";
  import ImageSet from "./Image.svelte";
  import { intersect } from "@svelte-put/intersect";

  const {
    imageSrc,
    imagePath,
    id,
    isFullScreen,
    containerClass,
    onIntersect,
    class: className,
    children,
    background,
    ...restProps
  } = $props();

  const changeHash = () => {
    const currentHash = window.location.hash;
    const newHash = `#${id}`;

    if (newHash !== currentHash) {
      history.replaceState(null, null, newHash);
      window.dispatchEvent(new HashChangeEvent("hashchange"));
    }
  };
  let isIntersecting = false;
  const onInnerIntersect = (event) => {
    const { entries } = event.detail;
    const entry = entries[0];
    isIntersecting = entry.isIntersecting;
  };
  const onIntersectCap = (event) => {
    const { entries } = event.detail;
    const entry = entries[0];
    if (entry.isIntersecting) {
      changeHash();
    }
    onIntersect?.(event);
  };
</script>

<section
  class={cn(
    "relative banner overflow-hidden w-full",
    isFullScreen ? "h-screen" : "h-auto",
    className
  )}
  {id}
  {...restProps}
  use:intersect={{ threshold: 0.1 }}
  onintersectonce={onInnerIntersect}
  onintersect={onIntersectCap}
>

  {#if background}
    {@render background?.()}
  {:else}
    <ImageSet class="inset-0 absolute" srcList={imageSrc} {imagePath} />
  {/if}

  <div
    class="relative p-4 lg:p-16 2xl:pl-24 w-auto block z-2 overflow-hidden {isFullScreen
      ? 'h-full'
      : ''}"
  >
    <div
      class="max-w-screen-2xl mx-auto ml-auto mr-auto w-full {containerClass}"
    >
      {#if children}
        {@render children?.()}
      {:else}
        <div class="block px-6 py-4 rounded-2"></div>
      {/if}
    </div>
  </div>
</section>
