<script>
    import { cn } from "$lib/utils";
    import CalendarIcon from "@lucide/svelte/icons/calendar";

    import Tag from "$lib/components/post-content-layout/tag/tag.svelte";
    import { headerIdResolver } from "$lib/components/post-content-layout/block/common";
    const {
        id,
        title,
        title_slot,
        content,
        content_slot,
        post_date,
        link,
        tags,
        serie,
        ...rest
    } = $props();
</script>

<article
    id={headerIdResolver("post-card", id)}
    {title}
    class={cn(["block border-b pb-6", rest?.class])}
    {...rest}
>
    <div class="group">
        <a
            class="block font-serif text-xl font-bold mb-2 group-hover:underline capitalize"
            href={link}
            target="_self"
        >
            {title}
        </a>

        <div class="flex items-center gap-2 text-muted-foreground my-3">
            {#if serie}
                <a
                    class="block text-sm text-slate-800 hover:text-slate-500 hover:underline"
                    href={serie.url}
                    target="_self"
                >
                    {serie.name}
                </a>
                <i class="h-4 border-r-1 border-r-solid -mb-1"></i>
            {/if}
            <div class="flex items-center gap-1">
                <CalendarIcon class="h-3 w-3" />
                <span class="text-sm">{post_date}</span>
            </div>
        </div>

        <p class="font-serif block line-clamp-3 my-3">
            {content}
        </p>

        <div class="flex gap-2">
            {#each tags as tag}
                <Tag {...tag} />
            {/each}
        </div>
    </div>
</article>
