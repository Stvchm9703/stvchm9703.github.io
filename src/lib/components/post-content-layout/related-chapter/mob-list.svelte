<script>
    let { relatedChapters } = $props();
</script>

<div class="lg:hidden mb-6 print:hidden">
    <details class="border p-4">
        <summary class="font-serif text-lg font-bold cursor-pointer"
            >In This Series</summary
        >
        <div class="mt-3 space-y-4">
            <!-- {post.relatedChapters.map((chapter) => ( -->
            {#each relatedChapters as chapter}
                <div key={chapter.id} class="text-sm">
                    <a
                        href={`/chapter/${chapter.id}`}
                        class="font-medium hover:underline"
                    >
                        {chapter.title}
                    </a>
                    <p class="text-muted-foreground text-xs mt-1">
                        {chapter.date}
                    </p>
                </div>
            {/each}
            <!-- ))} -->
        </div>
    </details>
</div>
