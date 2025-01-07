<script>
  import { cn } from "$lib/utils.js";

  let { class: className, ...restProps } = $props();
</script>

<div
  class={cn(
    "bg-card text-card-foreground rounded-lg border shadow-sm",
    className
  )}
  {...restProps}
>
  {@render children?.()}
</div>
