<script lang="ts" module>
  import type { HTMLAttributes } from "svelte/elements";
  import { cn, type WithElementRef } from "$lib/utils.js";
  export const stComponent = "card";
  export type Props = {
    isElement: {
      type: String;
      default: "div";
    };
  } & WithElementRef<HTMLAttributes<HTMLDivElement>>;
</script>

<script lang="ts">
  // import type { HTMLAttributes } from "svelte/elements";
  // import { cn, type WithElementRef } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    children,
    isElement,
    ...restProps
  }: WithElementRef<HTMLAttributes<HTMLDivElement>> & {
    isElement: string;
  } = $props();
</script>

<svelte:element
  this={isElement}
  bind:this={ref}
  data-slot="card"
  class={cn(
    "bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm",
    className
  )}
  {...restProps}
>
  {@render children?.()}
</svelte:element>
