<script>
  import { onMount } from "svelte";
  import { Picture } from "svelte-lazy-loader";
  import ImageSet from "./Image.svelte";
  import { intersect } from "@svelte-put/intersect";

  export let imageSrc = [];
  export let imagePath = "";
  export let id = "";
  export let ref = null;

  export let isFullScreen = false;
  export let containerClass = "";

  let isIntersecting = false;
  const onIntersect = (event) => {
    const { entries } = event.detail;
    const entry = entries[0];
    isIntersecting = entry.isIntersecting;
  };
</script>

<section
  class="relative banner overflow-hidden w-full {isFullScreen
    ? 'h-screen'
    : 'h-auto'} {$$props.class}"
  {id}
  use:intersect={{ threshold: 0.1 }}
  on:intersectonce={onIntersect}
>
  <slot name="background">
    <ImageSet class="inset-0 absolute" srcList={imageSrc} {imagePath} />
  </slot>

  <div
    class="absolute inset-y-0 sm:left-8 md:inset-16 md:left-24 right-2 block z-2 overflow-hidden"
  >
    <div class="max-w-screen-2xl mx-auto w-full {containerClass}">
      <slot>
        <div class="block px-6 py-4 rounded-2">
          <h1 class="text-light-50">Welcome to SvelteKit</h1>
          <p class="text-light-200">
            Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the
            documentation
          </p>
        </div>
      </slot>
    </div>
  </div>
</section>
