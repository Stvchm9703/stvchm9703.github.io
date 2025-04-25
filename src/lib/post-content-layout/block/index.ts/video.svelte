<script>
    const { block, ...rest } = $props();
</script>

<figure class="relative aspect-video w-full my-6" {...rest}>
    <video src={block.url} controls class="object-cover w-full h-full">
        <track kind="captions" />
    </video>
    <figcaption class="text-sm text-muted-foreground mt-2 italic">
        {block.caption}
    </figcaption>
</figure>
