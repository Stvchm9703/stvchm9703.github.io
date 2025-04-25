<script>
    const { id, name } = $props();
</script>

<a
    href={`/posts/tags/${name.toLowerCase().replace(/\s+/g, "-")}`}
    key={id}
    class="bg-muted text-xs px-2 py-1 rounded hover:bg-slate-100 hover:shadow-sm"
>
    {name}
</a>
