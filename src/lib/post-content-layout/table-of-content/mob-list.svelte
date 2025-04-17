<script>
    let { tableOfContents } = $props();
</script>

<div class="lg:hidden mb-6">
    <details class="border p-4">
        <summary class="font-serif text-lg font-bold cursor-pointer"
            >Table of Contents</summary
        >
        <nav class="mt-3 space-y-2">
            <!-- {post.tableOfContents.map((item) => ( -->
            {#each tableOfContents as item}
                <a
                    key={item.id}
                    href={`#${item.id}`}
                    class="block text-sm hover:underline py-1"
                >
                    {item.title}
                </a>
            {/each}
            <!-- ))} -->
        </nav>
    </details>
</div>
