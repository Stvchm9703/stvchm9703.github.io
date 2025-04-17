<script>
    const { block, ...rest } = $props();
</script>

<blockquote class="border-l-4 border-gray-300 pl-4 italic my-6" {...rest}>
    <p>"{block.content}"</p>
    {#if block.author}
        <footer class="text-sm text-muted-foreground mt-2">
            — {block.author}
        </footer>
    {/if}
</blockquote>
