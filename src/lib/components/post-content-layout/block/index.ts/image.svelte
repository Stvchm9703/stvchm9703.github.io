<script>
    const { block, ...rest } = $props();
</script>

<figure class="relative aspect-video w-full my-6" {...rest}>
    <img src={block.url} alt={block.alt} class="object-cover w-full h-full" />
    <figcaption class="text-sm text-muted-foreground mt-2 italic">
        {block.caption}
    </figcaption>
</figure>
