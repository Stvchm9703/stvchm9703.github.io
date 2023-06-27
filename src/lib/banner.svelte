<script>
  export let imageSrc = "";
  export let isFullScreen = false;
  // export let justify = "left";
  export let containerClass = "";
</script>

<section
  class="relative banner"
  class:h-screen={isFullScreen}
  class:h-auto={!isFullScreen}
>
  <slot name="background">
    <picture class="inset-0 absolute">
      <img
        class="object-cover w-full h-full"
        decoding="async"
        data-src={imageSrc}
        type="image/webp"
        src={imageSrc}
      />
    </picture>
  </slot>

  <div
    class="absolute inset-0 md:inset-16 grid items-center z-2 overflow-hidden uno:{containerClass}"
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
