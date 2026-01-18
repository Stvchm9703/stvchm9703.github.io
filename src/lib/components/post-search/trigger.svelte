<script>
  import { cn } from "$lib/utils";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Search } from "@lucide/svelte";
  const props = $props();
</script>

<Button
  variant="outline"
  class={cn(
    "relative h-9 w-full justify-start rounded-none bg-muted/50 text-sm text-muted-foreground sm:w-64 md:w-80"
  )}
  {...props}
>
  <Search class="mr-2 h-4 w-4 shrink-0 opacity-50" />
  <span class="inline-flex items-center gap-2"> Search... </span>
  <kbd
    class="pointer-events-none absolute right-2 top-1/2 -translate-y-1/2 hidden h-5 select-none items-center gap-1 rounded border bg-background px-1.5 font-mono text-xs font-medium opacity-100 sm:flex"
  >
    <span class="text-xs">⌘</span>K
  </kbd>
</Button>
