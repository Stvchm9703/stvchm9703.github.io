<script>
    let { relatedChapters } = $props();
</script>

<div class="border p-4">
    <h4 class="font-serif text-lg font-bold mb-3 border-b pb-2">
        In This Series
    </h4>
    <div class="space-y-4">
        {#each relatedChapters as chapter}
            <div key={chapter.id} class="text-sm">
                <a
                    href={`/chapter/${chapter.id}`}
                    class="font-medium hover:underline"
                >
                    {chapter.title}
                </a>
                <p class="text-muted-foreground text-xs mt-1">{chapter.date}</p>
            </div>
        {/each}
    </div>
</div>
