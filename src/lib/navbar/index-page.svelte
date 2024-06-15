<script>
  import { createSuspense, Suspense } from "@svelte-drama/suspense";
  import MediaQuery from "svelte-media-queries";
  // const suspend = createSuspense();
  // import Navbar from "$lib/navbar/index-set/main.svelte";

  const PCNavComponent = import("$lib/navbar/index-set/main.svelte").then(
    (m) => m.default
  );
  const MobNavComponent = import("$lib/navbar/mobile-set/main.svelte").then(
    (m) => m.default
  );
  const mediaQuesySet = ["(max-width: 82rem)", "(min-width: 82rem)"];
</script>

<Suspense let:suspend>
  <MediaQuery query={mediaQuesySet} let:matches>
    {@const [tablet, desktop] = matches}
    {#if tablet}
      {#await suspend(MobNavComponent) then MyComponent}
        <MyComponent />
      {/await}
    {:else if desktop}
      {#await suspend(PCNavComponent) then MyComponent}
        <MyComponent />
      {/await}
    {/if}
  </MediaQuery>
</Suspense>
