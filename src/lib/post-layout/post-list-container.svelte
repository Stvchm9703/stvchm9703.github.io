<script>
    import { cn } from "$lib/utils";
    const { class: className, children, title, serie, ...restProps } = $props();

</script>

<div
    class={cn("flex flex-col gap-3 w-full [&>div]:w-full", className)}
    {...restProps}
>
    <h2 class={cn(['pb-4'])}>
        <a class={cn(['font-serif text-lg font-bold capitalize'])} href="/posts/series/{serie}">
            {title}
        </a>
    </h2>
    {@render children?.()}
</div>
