<script>
    const { block, ...rest } = $props();
</script>

<h3 id={block.id} class="text-2xl font-bold mt-8 mb-4 scroll-mt-16" {...rest}>
    {block.content}
</h3>
