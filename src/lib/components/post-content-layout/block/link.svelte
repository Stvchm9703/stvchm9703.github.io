<script lang="ts">
  import {
    Card,
    CardContent,
    CardTitle,
    CardDescription,
  } from "$lib/components/ui/card";
  import type { ContentBlock } from "$generateor/content_block";
  import Tag from "$lib/components/post-content-layout/tag/tag.svelte";
  import { resolveMarks, resolveStyle, headerIdResolver } from "./common";

  let { class: className, id, componentAttr, componentType } = $props();

  const cardTitle = $derived(
    componentAttr?.title || componentAttr?.href || componentAttr?.url || "Link"
  );

  const linkUrl = $derived(componentAttr?.href || componentAttr?.url || "#");
</script>

{#if componentAttr["cardStyle"] === "Card"}
  <Card
    id={headerIdResolver(componentType, id)}
    isElement="a"
    stComponent={componentType}
    href={linkUrl}
    class={[resolveStyle(componentAttr["style"]), "my-2", className]}
    target="_blank"
  >
    <CardContent class={["py-4", "flex"]}>
      <img alt={componentAttr.title} class="w-8 h-8 rounded-full mr-4" />
      <div class="flex-grow flex-col">
        <CardTitle class="text-sm font-semibold">{cardTitle}</CardTitle>
        <CardDescription class="flex flex-col ">
          <p class="text-sm font-normal">
            {componentAttr.description}
          </p>
          <div class="flex flex-row gap-4">
            <span class="text-xs font-normal">
              {componentAttr.date}
            </span>
            {#each componentAttr.tags as tag}
              <Tag {...tag} />
            {/each}
          </div>
        </CardDescription>
      </div>
    </CardContent>
  </Card>
{/if}
