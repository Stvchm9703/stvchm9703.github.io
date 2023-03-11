<script lang="ts">
  import Image from "$lib/Image.svelte";
  import Navbar from "$lib/navbar/deault.svelte";
  import SampleImage from "$assets/img/placeholder-hero.jpg?webp&w=2400&q=85";
  import Banner from "$lib/banner.svelte";
  import Footer from "$lib/footer.svelte";
  console.log(SampleImage);
</script>

<svelte:head>
  <title>Steven, the web dev</title>
  <meta />
</svelte:head>
<Banner isFullScreen imageSrc={SampleImage}>
  <div class="block px-6 py-4 rounded-2 filter-blur-xl filter-brightness-70 ">
    <h1 class="text-light-50 text-5xl">Build for build</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </div>
</Banner>

<Navbar />

<Banner>
  <canvas slot="background" class="w-full h-screen inset-0 bg-dark-100" />
  <div
    class="block px-6 py-4 rounded-2 filter-blur-xl filter-brightness-70 bg-indigo-200 justify-self-end"
  >
    <h1 class="text-light-50 text-3xl">Welcome to SvelteKit</h1>
    <p class="text-light-200">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </div>
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
