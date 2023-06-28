<script>
  // import Image from "$lib/Image.svelte";
  import Navbar from "$lib/navbar/deault.svelte";
  import Banner from "$lib/banner.svelte";
  import BannerGridCell from "$lib/banner-grid-cell.svelte";

  import Footer from "$lib/footer.svelte";
  import SampleImage from "$assets/img/placeholder-hero.jpg?format=webp&w=750;900;1200;2400&q=85&as=metadata";
  import IndexImage from "$assets/img/trialScene4.png?w=750;900;1200;2400&q=85&format=webp&as=metadata";
  import CanvasRender from "$lib/canvas-render.svelte";
</script>

<svelte:head>
  <title>Steven, the web dev</title>
  <meta />
</svelte:head>
<Banner isFullScreen imageSrc={IndexImage}>
  <div
    class="block px-6 py-4 rounded-2 backdrop-filter backdrop-blur-xl backdrop-brightness-80"
  >
    <h1 class="text-light-50 text-5xl">Build for build</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </div>
</Banner>

<Navbar />

<Banner
  containerClass="grid-cols-2 grid-rows-auto grid-content-start gap-2 md:gap-4  lg:gap-unset
  not-first-children:mt-25vh 
  "
>
  <div slot="background" class="w-full h-200vh lg:h-300vh inset-0 bg-dark-100">
    <CanvasRender class="sticky inset-0 h-screen w-screen" />
  </div>
  <BannerGridCell align="left" class=" grid-row-start-1">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
  <BannerGridCell align="right" class=" grid-row-start-2">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
  <BannerGridCell align="left" class="grid-row-start-3">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
  <BannerGridCell align="right" class="grid-row-start-4">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
</Banner>

<Banner imageSrc={SampleImage} isFullScreen>
  <div class="flex flex-col align-stretch self-center">
    <div class="px-6 py-4">
      <h1 class="text-3xl text-light-50">Skill Set</h1>
      <p class="text-light-200">
        Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
      </p>
    </div>
    <div class="px-6 relative w-full overflow-y-scroll">
      <ul class="grid relative gap-1rem grid-flow-col grid-rows-2">
        <li class="block w-24rem bg-blue-700 rounded-sm h-5rem" />
        <li class="block w-24rem bg-blue-700 rounded-sm h-5rem" />
        <li class="block w-24rem bg-blue-700 rounded-sm h-5rem" />
        <li class="block w-24rem bg-blue-700 rounded-sm h-5rem" />
        <li class="block w-24rem bg-blue-700 rounded-sm h-5rem" />
        <li class="block w-24rem bg-blue-700 rounded-sm h-5rem" />
        <li class="block w-24rem bg-blue-700 rounded-sm h-5rem" />
      </ul>
    </div>
  </div>
</Banner>

<Footer />
