<script>
    let { tableOfContents } = $props();
</script>

<details class="border p-4 block lg:hidden mb-4 print:hidden">
    <summary class="font-serif text-sm font-bold cursor-pointer"
        >Table of Contents</summary
    >
    <nav class="mt-3 space-y-2">
        <!-- {post.tableOfContents.map((item) => ( -->
        {#each tableOfContents as item}
            <a
                key={item.id}
                href={item.url}
                class="block text-sm hover:underline py-1"
            >
                {item.label}
            </a>
        {/each}
        <!-- ))} -->
    </nav>
</details>
