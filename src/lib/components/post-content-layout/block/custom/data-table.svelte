<script lang="ts">
  import type { TableSchema } from "$generateor/content_block";
  import { cn } from "$lib/utils";

  const { tableSchema, name }: { tableSchema: TableSchema; name?: string } = $props();

  const fields = $derived(tableSchema?.schema?.fields ?? []);
  const data = $derived(tableSchema?.data ?? []);
</script>

<div class="overflow-x-auto w-full my-2">
  {#if name}
    <p class="text-xs text-gray-500 mb-1 font-mono">{name}</p>
  {/if}
  {#if fields.length > 0}
    <table class="w-full text-sm border-collapse">
      <thead>
        <tr class="border-b border-gray-300 bg-gray-50">
          {#each fields as field}
            <th
              class={cn([
                "px-3",
                "py-2",
                "text-left",
                "font-semibold",
                "text-nowrap",
                "break-keep",
                "text-gray-700",
              ])}
            >
              {field.name}
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each data as row, rowIdx}
          <tr
            class={cn([
              "border-b",
              "border-gray-200",
              "hover:bg-gray-50",
              "transition-colors",
              rowIdx % 2 === 1 ? "bg-gray-50/50" : "",
            ])}
          >
            {#each fields as field}
              <td class="px-3 py-1.5 text-gray-800">
                {row[field.name] ?? ""}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
    <p class="text-xs text-gray-400 mt-1 text-right">
      {data.length} row{data.length !== 1 ? "s" : ""}
    </p>
  {:else}
    <p class="text-sm text-gray-400 italic">No data available.</p>
  {/if}
</div>
