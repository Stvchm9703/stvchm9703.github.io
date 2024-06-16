<script>
  import ImageSet from "$lib/Image.svelte";
  import { onMount } from "svelte";

  export let url = "https://stvchm9703.itch.io/shadow-expeller";

  let url_image = "";

  onMount(async () => {
  
  });
</script>

<a class="block rounded-lg p-2 accent-coolGray/15">
  <ImageSet class="rounded" imagePath={url_image} />
</a>
