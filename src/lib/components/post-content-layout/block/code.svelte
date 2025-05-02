<script lang="ts">
    import { Highlight, LineNumbers } from "svelte-highlight";
    import Copy from "svelte-radix/Copy.svelte";
    import { typescript, type LanguageType } from "svelte-highlight/languages";
    import "svelte-highlight/styles/gruvbox-dark-soft.css";
    import { headerIdResolver, resolveStyle } from "./common";
    import type { ContentBlock } from "$generateor/content_block";
    import { Button } from "$lib/components/ui/button";
    import { onMount } from "svelte";

    const {
        id,
        fields,
        componentAttr,
        style: content_style,
        ...rest
    }: ContentBlock = $props();

    const { text, style: element_style, marks, ...other } = componentAttr;
    const hasText = text !== undefined && text !== "" && text !== null;

    let lang: LanguageType = $state.raw(typescript);

    onMount(async () => {
        // console.log("mounted");
        // lang = await import(`svelte-highlight/languages/${fields["lang"]}`)
        //     .default;
        // console.log(lang);
    });

    const copy = async () => {
        await navigator.clipboard?.writeText(text);
    };
</script>

<!-- <svelte:head>
    {#if hasText}
        {@html horizonDark}
    {/if}
</svelte:head> -->
<figure
    id={headerIdResolver(element_style, id)}
    class={[
        resolveStyle(content_style),
        "font-mono",
        "text-sm",
        "relative",
        "rounded-md",
        "overflow-hidden",
        "my-2",
    ]}
>
    {#if fields["from"]}
        <figcaption
            data-rehype-pretty-code-title=""
            data-language={fields["lang"]}
        >
            {fields["from"]}
        </figcaption>
    {/if}
    <Highlight language={lang} code={text} let:highlighted>
        <LineNumbers
            {highlighted}
            --line-number-color="rgba(255, 255, 255, 0.3)"
            --line-number-font-size="8px"
            --border-color="rgba(255, 255, 255, 0.1)"
            --padding-left="1em"
            --padding-right="1em"
        />
    </Highlight>

    <Button
        variant="outline"
        size="icon"
        class="absolute right-1 top-3 z-1 text-light-200 h-8 w-8 rounded-none"
        onclick={copy}
    >
        <Copy class="h-4 w-4" />
    </Button>
</figure>

<style>
    @import "$lib/styles/svelte_highlight.css";
</style>
