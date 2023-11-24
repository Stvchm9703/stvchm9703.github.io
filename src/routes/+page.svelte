<script>
  import Navbar from "$lib/navbar/index-page.svelte";
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
<Navbar />

<Banner id="index" isFullScreen imageSrc={IndexImage} containerClass="h-full grid items-center content-center">
  <BannerGridCell align="left">
    <h1 class="text-light-50 text-5xl">Build for build</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
</Banner>

<Banner
  id="introduction"
  containerClass="grid grid-cols-2 grid-rows-auto grid-content-start gap-2 md:gap-4 lg:gap-16"
>
  <div slot="background" class="w-full h-200vh lg:h-300vh inset-0 bg-dark-100">
    <CanvasRender class="sticky inset-0 h-screen w-screen" />
  </div>
  
  <BannerGridCell
    align="left"
    class=" grid-row-start-1 f-my-16-56"
  >
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
  <BannerGridCell align="right" class=" grid-row-start-2 f-my-16-56 ">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
  <BannerGridCell align="left" class="grid-row-start-3 f-my-16-56 ">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
  <BannerGridCell align="right" class="grid-row-start-4 f-my-16-56 ">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
</Banner>

<Banner id="skill" imageSrc={SampleImage} isFullScreen>
  <div class="flex flex-col align-stretch self-center">
    <div class="px-6 py-4">
      <h1 class="text-3xl text-light-50">Skill Set</h1>
      <p class="text-light-200">
        Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
      </p>
    </div>
    <div class="px-6 relative h-auto py-2 w-full overflow-y-hidden scrollbar scrollbar-rounded scrollbar-w-1 ">
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

<Banner id="contact" containerClass="">
  <div slot="background" class=" bg-blue-900 flex flex-col min-h-150" />

  <div class="flex flex-col">
    <h1 class="text-center text-white font-light">contact me</h1>
  </div>
</Banner>

<Footer />
