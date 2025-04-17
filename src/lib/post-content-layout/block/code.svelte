<script>
    const { block, ...rest } = $props();
</script>

<pre class="bg-gray-800 text-white p-4 rounded-md overflow-x-auto" {...rest}>
    <code class="language-{block.language}">
    {block.content}
    </code>
</pre>
