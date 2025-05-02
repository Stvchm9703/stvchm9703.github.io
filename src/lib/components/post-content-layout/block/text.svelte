<script lang="ts">
    import type { ContentBlock } from "$generateor/content_block";
    import { Checkbox } from "$lib/components/ui/checkbox";
    import { Label } from "$lib/components/ui/label";
    import {
        Accordion,
        AccordionItem,
        AccordionContent,
        AccordionTrigger,
    } from "$lib/components/ui/accordion";
    import Self from "./text.svelte";
    import { cn } from "$lib/utils";
    import {
        resolveMarks,
        pathResolver,
        headerIdResolver,
        resolveStyle,
    } from "./common";

    const {
        id,
        fields,
        componentAttr,
        style: content_style,
        ...rest
    }: ContentBlock = $props();
    const { text, style: element_style, marks, ...other } = componentAttr;
    // const { content } = block;
    // console.log(other);
    //
    const header_classes = "text-2xl font-bold mt-8 mb-4 scroll-mt-16";

    const hasText = text !== undefined && text !== "" && text !== null;
    let tMarks = marks.marks || [];
</script>

{#if element_style === "Header1" && hasText}
    <h2
        id={headerIdResolver(text, id)}
        class={[header_classes, resolveStyle(content_style)]}
    >
        {@html resolveMarks(tMarks, text)}
    </h2>
{:else if element_style === "Header2" && hasText}
    <h3
        id={headerIdResolver(text, id)}
        class={[header_classes, resolveStyle(content_style)]}
    >
        {@html resolveMarks(tMarks, text)}
    </h3>
{:else if element_style === "Header3" && hasText}
    <h4
        id={headerIdResolver(text, id)}
        class={[header_classes, resolveStyle(content_style)]}
    >
        {@html resolveMarks(tMarks, text)}
    </h4>
{:else if element_style === "Header4" && hasText}
    <h5
        id={headerIdResolver(text, id)}
        class={[header_classes, resolveStyle(content_style)]}
    >
        {@html resolveMarks(tMarks, text)}
    </h5>
{:else if element_style === "Numbered"}
    <ol id={headerIdResolver("ol", id)} class="list-decimal ml-4">
        {#each other["items"] as item}
            <li
                id={headerIdResolver("ol", item.id)}
                class={resolveStyle(item.style)}
            >
                <!-- {@html item["text"]} -->
                {@html resolveMarks(item["marks"], item["text"])}
            </li>
        {/each}
    </ol>
{:else if element_style === "Marked"}
    <ul id={headerIdResolver("ul", id)} class="list-disc ml-4">
        {#each other["items"] as item}
            <li
                id={headerIdResolver("ul", item.id)}
                class={resolveStyle(item.style)}
            >
                <!-- {@html item["text"]} -->
                {@html resolveMarks(item["marks"], item["text"])}
            </li>
        {/each}
    </ul>
{:else if element_style === "Quote" && hasText}
    <blockquote
        id={headerIdResolver("quote", id)}
        class="border-l-4 border-gray-300 pl-4 italic my-6"
        {...rest}
    >
        <p class={resolveStyle(content_style)}>
            "{@html resolveMarks(tMarks, text)}"
        </p>
        {#if other.author}
            <footer class="text-sm text-muted-foreground mt-2">
                — {other.author}
            </footer>
        {/if}
    </blockquote>
{:else if element_style === "Callout" && hasText}
    <div
        id={headerIdResolver(element_style, id)}
        class={cn([
            resolveStyle(content_style),
            "my-6",
            "flex-col",
            "py-4",
            "px-6",
            "rounded-md",
            "bg-slate-300",
        ])}
    >
        <!-- {@debug element_style} -->
        <p>
            {@html resolveMarks(tMarks, text)}
        </p>
    </div>
{:else if element_style === "Checkbox" && hasText}
    <div class="items-center flex gap-3 my-6">
        <Checkbox
            id={headerIdResolver(element_style, id)}
            checked={other["checked"]}
            disabled
        />
        <div class="grid gap-1.5 leading-none">
            <Label
                for={headerIdResolver(element_style, id)}
                class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >
                {@html resolveMarks(tMarks, text)}
            </Label>
        </div>
    </div>
{:else if element_style === "Toggle"}
    <Accordion id={headerIdResolver(element_style, id)} class="w-full">
        <AccordionItem
            class="border-slate-300"
            value={headerIdResolver(element_style, id)}
        >
            <AccordionTrigger>
                {@html resolveMarks(tMarks, text)}
            </AccordionTrigger>
            <AccordionContent class="pl-3 lg:pl-8 pr-2">
                {#each other["items"] as item}
                    <Self {...item} />
                {/each}
            </AccordionContent>
        </AccordionItem>
    </Accordion>
{:else if element_style === undefined && hasText}
    <p
        id={headerIdResolver(element_style, id)}
        class={resolveStyle(content_style)}
    >
        <!-- {@debug element_style} -->
        {@html resolveMarks(tMarks, text)}
    </p>
{/if}
