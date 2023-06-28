<script>
  import { onMount } from "svelte";
  import { Picture } from "svelte-lazy-loader";
  export let imageSrc = [];
  export let imagePath = "";

  let baseSet =
    imageSrc && Array.isArray(imageSrc)
      ? (imageSrc.filter((elm) => elm.format == "webp").length > 0
          ? imageSrc.filter((elm) => elm.format == "webp")
          : imageSrc
        ).sort((a, b) => a.width - b.width)[0]
      : imageSrc
      ? imageSrc
      : null;
  let remapImageSrc =
    imageSrc && Array.isArray(imageSrc)
      ? imageSrc
          .sort((a, b) => a.width - b.width)
          .map((elm) => ({ ...elm, media: `(min-width: ${elm.width + 1}px)` }))
      : imageSrc
      ? [imageSrc]
      : [];

  export let isFullScreen = false;
  export let containerClass = "";
</script>

<section
  class={"relative banner " + $$props.class}
  class:h-screen={isFullScreen}
  class:h-auto={!isFullScreen}
>
  <slot name="background">
    {#if baseSet}
      <Picture
        classes="inset-0 absolute children:h-full children:w-full children:object-cover"
        src={baseSet.src}
      >
        {#each remapImageSrc as { src, format, media }}
          <source data-srcset={src} {media} type="image/{format}" />
        {/each}
      </Picture>
    {/if}
  </slot>

  <div
    class="absolute inset-0 md:inset-16 grid items-center z-2 overflow-hidden {containerClass}"
  >
    <slot>
      <div class="block px-6 py-4 rounded-2">
        <h1 class="text-light-50">Welcome to SvelteKit</h1>
        <p class="text-light-200">
          Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
        </p>
      </div>
    </slot>
  </div>
</section>
