<script lang="ts">
    import { scrollbar as presetScrollbar } from "$lib/components/preset";
    import { cn } from "$lib/utils";
    import { Tabs } from "bits-ui";
    import { Highlight, LineNumbers } from "svelte-highlight";
    import Markdown from "svelte-exmarkdown";
    import { gfmPlugin } from "svelte-exmarkdown/gfm";
    import "svelte-highlight/styles/gruvbox-dark-soft.css";
    import { headerIdResolver } from "../common";
    // import "$lib/styles/svelte_highlight.css";
    const exmdPlugins = [gfmPlugin()];

    const { id, componentAttr } = $props();
    const { cell, fileName, cellNumber } = componentAttr;
    const { source, outputs, cell_type } = cell;
    const elemId = headerIdResolver("jupyter", id);

    async function initHighlight() {
        if (cell_type === "code") {
            return await import(`svelte-highlight/languages/python`).then(
                (module) => module.default,
            );
        } else if (cell_type === "markdown") {
            return await import(`svelte-highlight/languages/markdown`).then(
                (module) => module.default,
            );
        }

        return null;
    }

    const tabContentClass = cn([
        presetScrollbar,
        // "lg:h-180",
        "h-auto",
        "aspect-3/2",
        "overflow-scroll",
        "font-sans text-sm",
        // "p-4  leading-normal"
    ]);

    const tabTriggerClass = cn([
        "h-8 rounded-sm bg-transparent py-2 px-4 data-[state=active]:shadow-mini dark:data-[state=active]:bg-muted  data-[state=active]:bg-white",
    ]);
</script>

<figure
    id={elemId}
    class="rounded-lg border-none bg-background-alt shadow-sm w-full border p-2 my-3"
>
    <Tabs.Root value="outputs">
        <Tabs.List
            class="rounded-t-md bg-coolgray-200 pb-1 shadow-mini-inset dark:bg-background  flex gap-1 p-1 text-sm font-semibold leading-[0.01em] font-sans dark:border dark:border-neutral-600/30"
        >
            <Tabs.Trigger value="outputs" class={tabTriggerClass}>
                Outputs
            </Tabs.Trigger>
            <Tabs.Trigger value="source" class={tabTriggerClass}>
                Source
            </Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="outputs" class={tabContentClass}>
            {#if cell_type === "code"}
                {#each outputs as outputBlock}
                    <section
                        class={cn([
                            "jupyter-cell",
                            "p-4  leading-normal",
                            // "[&_thead>_tr>_th]:(inline text-nowrap break-keep)",
                            outputBlock.data["image/png"]
                                ? "bg-coolGray-100 flex items-center justify-center"
                                : "bg-transparent",
                        ])}
                    >
                        {#if outputBlock.data["image/png"]}
                            <img
                                class="w-full aspect-3/2 block object-contain object-position-center"
                                src={`data:image/png;base64,${outputBlock.data["image/png"]}`}
                                alt="Image"
                                aria-label="render result, {fileName} - cell: {cellNumber}"
                            />
                            <figcaption>
                                {outputBlock.data["text/plain"]}
                                render result, {fileName} - cell: {cellNumber}
                            </figcaption>
                        {:else if outputBlock.data["text/html"]}
                            {@html outputBlock.data["text/html"].join("")}
                        {:else if outputBlock.data["text/plain"]}
                            {@html outputBlock.data["text/plain"].join("")}
                        {:else}
                            No data available
                        {/if}
                    </section>
                {/each}
            {:else if cell_type === "markdown"}
                <section class="jupyter-cell markdown block">
                    <Markdown md={source.join("")} plugins={exmdPlugins} />
                </section>
            {/if}
        </Tabs.Content>
        <Tabs.Content value="source" class={tabContentClass}>
            {#await initHighlight() then mod}
                <Highlight
                    language={mod}
                    code={source.join("")}
                    let:highlighted
                >
                    <LineNumbers
                        {highlighted}
                        --line-number-color="rgba(255, 255, 255, 0.3)"
                        --line-number-font-size="8px"
                        --border-color="rgba(255, 255, 255, 0.1)"
                        --padding-left="1em"
                        --padding-right="1em"
                    />
                </Highlight>
            {/await}
        </Tabs.Content>
    </Tabs.Root>

    <figcaption
        class="rounded-b-md bg-coolgray-200 text-sm font-semibold font-sans dark:border text-right dark:border-neutral-200 px-4 py-1 flex"
    >
        <span class="text-gray-500">File: {fileName} - cell: {cellNumber}</span>
    </figcaption>
    <!-- <pre><code>{code}</code></pre> -->
</figure>

<style>
    @import "$lib/styles/svelte_highlight.css";
    .jupyter-cell {
        /* @apply border-1 border-solid border-coolgray-200; */
        @apply w-auto content;
    }
    .jupyter-cell :global(th),
    .jupyter-cell :global(td) {
        @apply table-cell p-1;
    }
    .jupyter-cell :global(tr) {
        @apply border-b-1 border-solid border-coolgray-200 hover:bg-coolgray-100 transition-colors;
    }
    .jupyter-cell :global(thead th) {
        @apply text-nowrap break-keep;
    }
    .jupyter-cell :global(tbody th) {
        @apply text-right;
    }

    .jupyter-cell.markdown {
        @apply p-4 font-sans;
    }
    .jupyter-cell.markdown :global(h1) {
        @apply text-2xl font-bold mb-4;
    }
    .jupyter-cell.markdown :global(h2) {
        @apply text-xl font-bold mb-4;
    }
    .jupyter-cell.markdown :global(h3) {
        @apply text-lg font-bold mb-4;
    }
    .jupyter-cell.markdown :global(h4) {
        @apply font-bold mb-4;
    }
    .jupyter-cell.markdown :global(h5) {
        @apply font-bold mb-4;
    }
    .jupyter-cell.markdown :global(h6) {
        @apply font-bold mb-4;
    }

    .jupyter-cell.markdown :global(p),
    .jupyter-cell.markdown :global(td) {
        @apply text-sm leading-relaxed;
    }
</style>
