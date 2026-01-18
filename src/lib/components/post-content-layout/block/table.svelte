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
  import type { ContentBlock } from "$generateor/content_block";

  import {  resolveStyle } from "./common";

  const { id, order, fields, componentAttr, style, ...rest }: ContentBlock =
    $props();
  const componentType = rest?.componentType || rest?.layoutStyle || "Table";

  let items = componentAttr?.items || rest?.items || [];
  const header = items[1].items[0];
  const rows = items[1].items.slice(1);
</script>

<Table
  id={headerIdResolver(componentType + "_", id || "")}
  {...rest}
  class="table"
>
  <TableHeader>
    <TableRow>
      {#if header}
        {#each header.items as cell}
          <TableHead class={resolveStyle(cell.style)}>
            <Block {...cell} />
          </TableHead>
        {/each}
      {/if}
    </TableRow>
  </TableHeader>
  <TableBody>
    {#if rows}
      {#each rows as row}
        <TableRow>
          {#each row.items as cell}
            <TableCell class={resolveStyle(cell.style)}>
              <Block {...cell} />
            </TableCell>
          {/each}
        </TableRow>
      {/each}
    {/if}
  </TableBody>
</Table>
