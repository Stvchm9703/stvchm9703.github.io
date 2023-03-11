<script>
  export let imageSrc = "";
  export let isFullScreen = false;
  export let justify = "left";
</script>

<section class="relative banner">
  <slot name="background">
    <picture class="inset-0">
      <img
        class="object-cover w-full"
        class:h-auto={!isFullScreen}
        class:h-screen={isFullScreen}
        decoding="async"
        data-src={imageSrc}
        type="image/webp"
        src={imageSrc}
      />
    </picture>
  </slot>

  <div class="absolute inset-4rem grid items-center z-2 overflow-hidden">
    <slot>
      <div
        class="block px-6 py-4 rounded-2 filter-blur-xl filter-brightness-70 "
      >
        <h1 class="text-light-50">Welcome to SvelteKit</h1>
        <p class="text-light-200">
          Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
        </p>
      </div>
    </slot>
  </div>
</section>
