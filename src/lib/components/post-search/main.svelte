<script lang="ts">
  import * as Command from "$lib/components/ui/command/index.js";
  import { cn } from "$lib/utils";

  import SearchBar from "./search-bar.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Search } from "@lucide/svelte";
  import { onMount } from "svelte";
  //   import Trigger from "./trigger.svelte";

  let open = $state(false);

  function handleOnTriggerClick() {
    open = true;
    console.log("trigger clicked");
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      open = !open;
    }
  }
//   onMount(() => {
//     console.log("Post Search Mounted");
//   });
</script>

<svelte:document onkeydown={handleKeydown} />

<Button
  variant="outline"
  class={cn(
    "relative h-9 w-full justify-start rounded-none bg-muted/50 text-sm text-muted-foreground sm:w-64 md:w-80"
  )}
  onclick={handleOnTriggerClick}
>
  <Search class="mr-2 h-4 w-4 shrink-0 opacity-50" />
  <span class="inline-flex items-center gap-2"> Search... </span>
  <kbd
    class="pointer-events-none absolute right-2 top-1/2 -translate-y-1/2 hidden h-5 select-none items-center gap-1 rounded border bg-background px-1.5 font-mono text-xs font-medium opacity-100 sm:flex"
  >
    <span class="text-xs">⌘</span>K
  </kbd>
</Button>

<Command.CommandDialog bind:open class="bg-zinc-50 dark:bg-zinc-800 xl:max-w-xl">
  <SearchBar />
</Command.CommandDialog>
