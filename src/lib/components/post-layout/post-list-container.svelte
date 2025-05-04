<script>
    import { cn } from "$lib/utils";
    const { class: className, children, title, serie, url, ...restProps } = $props();

</script>

<div
    class={cn("flex flex-col gap-3 w-full [&>div]:w-full", "py-4", 'my-4',  className)}
    {...restProps}
>
    <div class={cn(['flex', 'justify-between', 'w-full'])}>
        <h2 class={cn(['pb-4', 'font-serif','text-lg','font-bold','capitalize'])}>
            {title}
        </h2>

        <a class={cn(['text-md', 'capitalize'])} href="{url}" >
            Read More
        </a>
    </div>
    {@render children?.()}

</div>
