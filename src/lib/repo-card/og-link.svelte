<script lang="ts">
  import ImageSet from "$lib/Image.svelte";
  import { onMount } from "svelte";

  interface Props {
    url?: string;
  }

  let { url = "https://stvchm9703.itch.io/shadow-expeller" }: Props = $props();

  let url_image = "";

  onMount(async () => {
  
  });
</script>

<a class="block rounded-lg p-2 accent-coolGray/15">
  <ImageSet class="rounded" imagePath={url_image} />
</a>
