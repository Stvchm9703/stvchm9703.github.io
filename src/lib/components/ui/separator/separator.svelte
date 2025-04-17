<script lang="ts">
    import { Separator as SeparatorPrimitive } from "bits-ui";
    import clsx from "clsx";

    let {
        ref = $bindable(null),
        class: className,
        orientation = "horizontal",
        ...restProps
    }: SeparatorPrimitive.RootProps = $props();
</script>

<SeparatorPrimitive.Root
    bind:ref
    class={clsx(
        "bg-border shrink-0",
        orientation === "horizontal" ? "h-[1px] w-full" : "min-h-full w-[1px]",
        className,
    )}
    {orientation}
    {...restProps}
/>
