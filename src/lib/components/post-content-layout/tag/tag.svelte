<script>
    import { cn } from "$lib/utils";
    import { badgeVariants } from "$lib/components/ui/badge";
    const { id, label : name, url, ...rest } = $props();
</script>

<a
    key={id}
    href={url}
    class={cn([
        badgeVariants({ variant: "default" }),
        "hover:bg-slate-100 hover:shadow-sm",
        "border-1 border-solid border-coolgray-200",
    ])}
    target="_self"
    {...rest}
>
    {name}
</a>
