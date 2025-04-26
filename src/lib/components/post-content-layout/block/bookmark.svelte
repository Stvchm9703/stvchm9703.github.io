<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import type { ContentBlock } from "$generateor/content_block";
    import { resolveMarks, resolveStyle, headerIdResolver } from "./common";

    let { class: className, id, componentAttr, ...rest } = $props();
</script>

<Card.Root
    id={headerIdResolver("bookmark", id)}
    isElement="a"
    stComponent="bookmark"
    href={componentAttr.url}
    class={[resolveStyle(componentAttr["style"]), "my-2", className]}
>
    <Card.Content class={["py-4"]}>
        <Card.Title>{componentAttr.title}</Card.Title>
        <Card.Description>{componentAttr.description}</Card.Description>
    </Card.Content>
</Card.Root>
