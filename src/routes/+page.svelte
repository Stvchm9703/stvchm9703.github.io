<script lang="ts">
  // import Image from '$lib/Image.svelte'
  import SampleImage from "$assets/img/placeholder-hero.jpg?webp&w=2400&q=85";
</script>

<h1>Welcome to SvelteKit</h1>
<p class="bg-dark-50 text-light-100 ">
  Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
</p>

<div class="w-full aspect-16/9 w-full relative >*:pb-[56.5%]">
  <img class="w-full" src={SampleImage} alt="" type="webp" />
  <!-- <Image src={"placeholder-hero.jpg"} /> -->
</div>
