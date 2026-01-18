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
  import { uniqBy } from "lodash-es";

  import { resolveMarks, resolveStyle } from "./common";

  const { id, order, fields, componentAttr, style,  ...rest
  }: ContentBlock = $props();
  const componentType = rest?.componentType || rest?.layoutStyle || "Table";

  let items = componentAttr?.items || rest?.items || [];
//   let tableItems = uniqBy(items, "order");
// //   let header = tableItems.find((i) => i.layoutStyle === "TableColumns");
//   let _rows = tableItems.find((i) => i.layoutStyle === "TableRows");
//   let header = null;
//     let rows = [];
//   if (!!_rows) {
//     header = _rows.items[0];
//     rows = _rows.items.slice(1);
//   }
 
//   console.log(" tableItems");
//   console.log("header : ", header);
//   console.log("row : ", rows);
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
    {#each rows as row}
      <TableRow>
        {#each row.items as cell}
          <TableCell class={resolveStyle(cell.style)}>
            <Block {...cell} />
          </TableCell>
        {/each}
      </TableRow>
    {/each}
  </TableBody>
</Table>
