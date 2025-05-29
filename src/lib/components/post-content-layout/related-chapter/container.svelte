<script>
    let { relatedChapters } = $props();
</script>

<div class="border p-4 print:hidden">
    <h4 class="font-serif text-lg font-bold mb-3 border-b pb-2">
        In This Series
    </h4>
    <div class="space-y-4">
        {#each relatedChapters as chapter}
            <div key={chapter.id} class="text-sm">
                <a
                    href={chapter.url}
                    class="font-medium hover:underline"
                    target="_self"
                >
                    {chapter.label}
                </a>
            </div>
        {/each}
    </div>
</div>
