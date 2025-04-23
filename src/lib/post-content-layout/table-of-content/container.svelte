<script>
    // import Link from "./link.svelte";
    let { tableOfContents } = $props();
</script>

<div class="border p-4 mb-6 bg-muted/30">
    <h4 class="font-serif text-lg font-bold mb-3 border-b pb-2">
        Table of Contents
    </h4>
    <nav class="space-y-2">
        <!-- {post.tableOfContents.map((item) => ( -->
        {#each tableOfContents as item}
            <a
                key={item.id}
                href={item.url}
                class={"block text-sm hover:underline py-1 border-l-2 border-transparent hover:border-primary pl-2 ransition-colors"}
                style={`padding-left: ${item.level * 0.75}rem;`}
            >
                {item.title}
            </a>
        {/each}
    </nav>
</div>
