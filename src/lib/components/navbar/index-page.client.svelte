<script lang="ts">
  import { Suspense } from "@svelte-drama/suspense";
  import MediaQuery from "svelte-media-queries";
  // const suspend = createSuspense();


  interface Props {
    // import Navbar from "$lib/navbar/index-set/main.svelte";
    activeId: any;
  }

  let { activeId }: Props = $props();
  const PCNavComponent = import("$lib/components/navbar/index-set/main.svelte").then(
    (m) => m.default
  );
  const MobNavComponent = import("$lib/components/navbar/mobile-set-2/main.svelte").then(
    (m) => m.default
  );
  const mediaQuesySet = ["(max-width: 82rem)", "(min-width: 82rem)"];

</script>

<Suspense>
  {#snippet children({ suspend })}
    <MediaQuery query={mediaQuesySet} >
      {#snippet children({ matches })}
        {@const [tablet, desktop] = matches}
        {#if tablet}
          {#await suspend(MobNavComponent) then MobComponent}
            <MobComponent {activeId} />
          {/await}
        {:else if desktop}
          {#await suspend(PCNavComponent) then PcComponent}
            <PcComponent />
          {/await}
        {/if}
            {/snippet}
    </MediaQuery>
  {/snippet}
</Suspense>
