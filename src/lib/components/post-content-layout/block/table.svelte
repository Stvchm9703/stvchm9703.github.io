<script lang="ts">
  import { headerIdResolver } from "./common";

  import {
    Table,
    TableHeader,
    TableHead,
    TableBody,
    TableRow,
    TableCell,
  } from "$lib/components/ui/table";
  import Block from "./layout.svelte";
  import type { ContentBlock, LayoutComponentAttr, LayoutItem } from "$generateor/content_block";

  import { resolveStyle } from "./common";

  const { id, order, fields, componentAttr, style, ...rest }: ContentBlock = $props();

  // Narrow to LayoutComponentAttr — this component is only mounted for Table/Layout blocks
  const layoutAttr = $derived(
    (componentAttr.componentType === "Table" || componentAttr.componentType === "Layout" || componentAttr.componentType === undefined)
      ? (componentAttr as LayoutComponentAttr)
      : null
  );

  const componentType = $derived(
    layoutAttr?.componentType || layoutAttr?.layoutStyle || "Table"
  );

  const items = $derived(layoutAttr?.items ?? []);

  const header = $derived(
    items.length > 1 && "items" in items[1] ? (items[1] as LayoutComponentAttr) : null
  );
  const headerItems = $derived(
    header && "items" in header ? (header.items ?? []) : []
  );
  const rows = $derived(
    items.length > 1 ? items.slice(2) : []
  );
</script>

<Table
  id={headerIdResolver((componentType ?? "Table") + "_", id || "")}
  {...rest}
  class="table"
>
  <TableHeader>
    <TableRow>
      {#if header && headerItems.length > 0}
        {#each headerItems as cell}
          {#if "componentAttr" in cell}
            <TableHead class={resolveStyle(cell.style)}>
              <Block {...cell} />
            </TableHead>
          {/if}
        {/each}
      {/if}
    </TableRow>
  </TableHeader>
  <TableBody>
    {#each rows as row}
      {#if "items" in row && row.items}
        <TableRow>
          {#each row.items as cell}
            {#if "componentAttr" in cell}
              <TableCell class={resolveStyle(cell.style)}>
                <Block {...cell} />
              </TableCell>
            {/if}
          {/each}
        </TableRow>
      {/if}
    {/each}
  </TableBody>
</Table>
