<script lang="ts">
    import * as Table from "$lib/components/ui/table";
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

<Table.Root {...rest} {...other} class="table">
    <Table.Header>
        <Table.Row>
            {#each tableData.rowData[0].cells as item}
                <Table.Head class={resolveStyle(item.style)}
                    >{@html reresolveMarks(item)}</Table.Head
                >
            {/each}
        </Table.Row>
    </Table.Header>
    <Table.Body>
        {#each tableData.rowData.slice(1) as row}
            <Table.Row>
                {#each row.cells as cell}
                    <Table.Cell class={resolveStyle(cell.style)}>
                        {@html reresolveMarks(cell)}
                    </Table.Cell>
                {/each}
            </Table.Row>
        {/each}
    </Table.Body>
</Table.Root>
