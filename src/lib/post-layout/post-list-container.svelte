<script>
    import { cn } from "$lib/utils.js";
    const { class: className, children, ...restProps } = $props();

</script>

<div
    class={cn("flex flex-col mx-8 gap-3 w-full [&>div]:w-full", className)}
    {...restProps}
>
    {@render children?.()}
</div>
