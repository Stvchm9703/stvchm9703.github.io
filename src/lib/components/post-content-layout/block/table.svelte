<script lang="ts">
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
    import { resolveMarks, resolveStyle } from "./common";

    const { id, fields, componentAttr, style, ...rest }: ContentBlock =
        $props();
    const { items, ...other } = componentAttr;

    const header = items[1].items[0];
    const rows = items[1].items.slice(1);
</script>

<Table {...rest} {...other} class="table">
    <TableHeader>
        <TableRow>
            {#each header.items as item}
                <TableHead class={resolveStyle(item.style)}>
                    <Block {...item} />
                </TableHead>
            {/each}
        </TableRow>
    </TableHeader>
    <TableBody>
        {#each rows as row}
            <TableRow>
                {#each row.items as cell}
                    <TableCell class={resolveStyle(cell.style)}>
                        <Block {...cell} />
                        <!-- {cell} -->
                    </TableCell>
                {/each}
            </TableRow>
        {/each}
    </TableBody>
</Table>
