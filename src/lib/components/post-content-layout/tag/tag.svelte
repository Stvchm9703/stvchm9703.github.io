<script>
    import { cn } from "$lib/utils";
    import { badgeVariants } from "$lib/components/ui/badge";
    const { id, name } = $props();
</script>

<a
    href={`/posts/tags/${name.toLowerCase().replace(/\s+/g, "-")}`}
    key={id}
    class={cn([
        badgeVariants({ variant: "default" }),
        "hover:bg-slate-100 hover:shadow-sm",
    ])}
>
    {name}
</a>
