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

    const { type: fileType, style, name: fileName } = componentAttr;
</script>

{#if fileType === "Image"}
    <figure class="w-full my-2">
        <figcaption></figcaption>
        <picture>
            <img src={`/blog/image/${fileName}`} />
        </picture>
    </figure>
{:else if fileType === "Video"}
    <figure class="w-full my-2 aspect-16/9">
        <figcaption></figcaption>
        <video>
            <img src={`/blog/image/${fileName}`} />
        </video>
    </figure>
{:else}
    {fileType}
{/if}
