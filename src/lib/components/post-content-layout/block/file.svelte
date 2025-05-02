<script lang="ts">
    import type { ContentBlock } from "$generateor/content_block";
    import { cn } from "$lib/utils";
    import { resolveStyle } from "./common";
    const {
        id,
        fields,
        componentAttr,
        style: content_style,
        ...rest
    }: ContentBlock = $props();

    const { type: fileType, style, name: fileName, fileUrl } = componentAttr;
    const baseClass = ["w-full", "my-2", "max-h-240"];
</script>

{#if fileType === "Image"}
    <figure class={cn([baseClass, resolveStyle(style)])}>
        <figcaption></figcaption>
        <picture>
            <img
                src={`/blog/assets/${fileUrl}`}
                alt="image"
                aria-label={fileName}
            />
        </picture>
    </figure>
{:else if fileType === "Video"}
    <figure class={cn([baseClass, "aspect-16/9", resolveStyle(style)])}>
        <figcaption></figcaption>
        <video src={`/blog/assets/${fileUrl}`}>
            <track kind="captions" />
        </video>
    </figure>
{/if}
