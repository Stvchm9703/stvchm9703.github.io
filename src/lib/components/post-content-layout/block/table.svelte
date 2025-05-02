<script lang="ts">
    import {
        Table,
        TableHeader,
        TableHead,
        TableBody,
        TableRow,
        TableCell,
    } from "$lib/components/ui/table";
    import type { ContentBlock } from "$generateor/content_block";
    import { resolveMarks, resolveStyle } from "./common";

    const { id, fields, componentAttr, style, ...rest }: ContentBlock =
        $props();
    const { tableData, ...other } = componentAttr;
    const reresolveMarks = (cell) => {
        const marks =
            cell.marks.length > 0 ? cell.marks : cell.marks.marks || [];
        return resolveMarks(marks, cell.text);
    };
</script>

<Table {...rest} {...other} class="table">
    <TableHeader>
        <TableRow>
            {#each tableData.rowData[0].cells as item}
                <TableHead class={resolveStyle(item.style)}
                    >{@html reresolveMarks(item)}</TableHead
                >
            {/each}
        </TableRow>
    </TableHeader>
    <TableBody>
        {#each tableData.rowData.slice(1) as row}
            <TableRow>
                {#each row.cells as cell}
                    <TableCell class={resolveStyle(cell.style)}>
                        {@html reresolveMarks(cell)}
                    </TableCell>
                {/each}
            </TableRow>
        {/each}
    </TableBody>
</Table>
