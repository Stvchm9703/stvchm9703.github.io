<script>
    let { relatedChapters } = $props();
</script>

<details class="block lg:hidden my-4 print:hidden border p-4">
    <summary class="font-serif text-sm font-bold cursor-pointer"
        >In This Series</summary
    >
    <div class="mt-3 space-y-4">
        <!-- {post.relatedChapters.map((chapter) => ( -->
        {#each relatedChapters as chapter}
            <div key={chapter.id} class="text-sm">
                <a
                    href={chapter.url}
                    class="font-medium hover:underline"
                    target="_self"
                >
                    {chapter.title}
                </a>
            </div>
        {/each}
        <!-- ))} -->
    </div>
</details>
