<script>
  import Navbar from "$lib/navbar/index-page.svelte";
  import Banner from "$lib/banner.svelte";
  import BannerGridCell from "$lib/banner-grid-cell.svelte";

  import Footer from "$lib/footer.svelte";
  import SampleImage from "$assets/img/placeholder-hero.jpg?format=webp&w=750;900;1200;2400&q=85&as=metadata";
  import IndexImage from "$assets/img/trialScene4.png?w=750;900;1200;2400&q=85&format=webp&as=metadata";
  import ScrollFrame from "$lib/scroll-frame.svelte";
  import SkillTree from "$lib/skill-tree.svelte";
  const testArray = Array.from({ length: 30 }, (_, i) => i + 1);
</script>

<svelte:head>
  <title>Steven, the web dev</title>
  <meta />
</svelte:head>
<Navbar />

<Banner
  id="index"
  isFullScreen
  imageSrc={IndexImage}
  containerClass="h-full grid items-center content-center"
>
  <BannerGridCell align="left">
    <h1 class="text-light-50 text-5xl leading-loose">Steven - Cheng Ho Man</h1>
    <p class="text-light-200 leading-loose">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
</Banner>

<Banner
  id="introduction"
  class="!h-500vh w-full "
  containerClass=" grid grid-cols-3 justify-items-stretch  grid-rows-auto grid-content-start gap-2 md:gap-4 lg:gap-16 pt-32"
>
  <ScrollFrame
    slot="background"
    class="bg-dark-100 inset-0 bg-dark-100 fixed -z-1 "
  />

  <h1
    class="grid-row-start-1 grid-col-start-2 grid-col-end-2 text-center text-light-50 text-3xl"
  >
    Welcome to SvelteKit
  </h1>
  {#each testArray as i}
    <BannerGridCell align={i % 2 === 0 ? "left" : "right"} row={i} class=" ">
      <h2 class="text-light-50 text-3xl">A Man who run with code</h2>
      <span>{i}</span>
      <p class="text-light-200">almost, but not quite,</p>
    </BannerGridCell>
  {/each}
  <!-- <BannerGridCell align="left" row={1} class=" ">
    <h2 class="text-light-50 text-3xl">A Man who run with code</h2>
    <p class="text-light-200">almost, but not quite,</p>
  </BannerGridCell> -->
  <!-- <BannerGridCell align="right" row={2} class=" ">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
  <BannerGridCell align="left" row={3} class=" ">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
  <BannerGridCell align="right" row={4} class=" ">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
  <hr id="project" class="block opacity-0 grid-row-start-5 grid-col-end-2" />
  <BannerGridCell align="right" row={5} class=" ">
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell> -->
</Banner>

<Banner id="skill" isFullScreen containerClass="">
  <div class="flex flex-col self-center max-w-full">
    <div class="px-6 py-4">
      <h1 class="text-3xl text-light-50">Skill Set</h1>
      <p class="text-light-200">
        Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
      </p>
    </div>
    <SkillTree />
  </div>
</Banner>

<Banner id="contact" containerClass="">
  <div slot="background" class=" bg-blue-900 flex flex-col min-h-150" />

  <div class="flex flex-col">
    <h1 class="text-center text-3xl text-light-50 font-light">Contact Me</h1>
  </div>
</Banner>

<Footer />
