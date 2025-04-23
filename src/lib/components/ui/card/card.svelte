<script lang="ts">
    import type { HTMLAttributes } from "svelte/elements";
    import { cn } from "$lib/utils.js";

    type $$Props = HTMLAttributes<HTMLDivElement>;

    let isElement: string | undefined = undefined;
    let className: $$Props["class"] = undefined;
    export { className as class, isElement };
</script>

<svelte:element
    this={isElement}
    class={cn(
        "bg-card text-card-foreground rounded-lg border shadow-sm block border-slate-300",
        className,
    )}
    {...$$restProps}
>
    <slot />
</svelte:element>
