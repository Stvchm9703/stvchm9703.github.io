<script lang="ts">
    import {
        Card,
        CardContent,
        CardTitle,
        CardDescription,
    } from "$lib/components/ui/card";
    // import type { ContentBlock } from "$generateor/content_block";
    import { resolveMarks, resolveStyle, headerIdResolver } from "./common";

    let { class: className, id, componentAttr, ...rest } = $props();
</script>

<Card
    id={headerIdResolver("bookmark", id)}
    isElement="a"
    stComponent="bookmark"
    href={componentAttr.url}
    class={[resolveStyle(componentAttr["style"]), "my-2", className]}
>
    <CardContent class={["py-4"]}>
        <CardTitle>{componentAttr.title}</CardTitle>
        <CardDescription>{componentAttr.description}</CardDescription>
    </CardContent>
</Card>
