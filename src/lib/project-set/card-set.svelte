<script>
  import { onMount } from "svelte";
  import { DateTime } from "luxon";
  let tmp = $state([]);
  onMount(async () => {
    let importData = (await import(`$assets/content/full-repo-data.json`)).default;
    console.log(importData);
    importData = importData.map((elm) => ({
      ...elm,
      updated_at_date: DateTime.fromISO(elm.updated_at).toUnixInteger(),
    }));
    importData = importData.sort((a, b) => b.updated_at_date - a.updated_at_date);
    tmp = importData.slice(0,11);
    console.log(tmp)
  });
</script>

<div class="grid gap-12">
  {#each tmp as item}
    <div>{item.name} {item.updated_at}</div>
  {/each}
</div>
