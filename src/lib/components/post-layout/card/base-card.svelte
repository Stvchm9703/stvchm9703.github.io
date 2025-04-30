<script>
    import { cn } from "$lib/utils";
    import Tag from "$lib/components/post-content-layout/tag/tag.svelte";
    let {
        title,
        title_slot,
        content,
        content_slot,
        post_date,
        link,
        tags,
        ...rest
    } = $props();
</script>

<article class={cn(["block border-b pb-6", rest?.class])} {...rest}>
    <a href={link} target="_blank" class="group">
        <h3
            class="block font-serif text-xl font-bold mb-2 group-hover:underline capitalize"
        >
            {title}
        </h3>

        <div class="flex items-center gap-2 text-muted-foreground my-3">
            <i class="h-3 w-3 i-carbon-calendar"></i>
            <span class="text-xs">{post_date}</span>
        </div>

        <p class="font-serif block line-clamp-3 my-3">
            {content}
        </p>

        <div class="flex gap-2">
            {#each tags as tag}
                <Tag {...tag} />
            {/each}
        </div>
    </a>
</article>
