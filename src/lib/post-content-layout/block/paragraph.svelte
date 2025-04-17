<script>
    const { block, ...rest } = $props();
    const { content } = block;
</script>

<p {...rest}>{@html content}</p>
