<script>
  export let data = [];
</script>

<table
  class="cds--data-table cds--data-table--no-border cds--data-table cds--data-table--no-border"
>
  <thead>
    <tr>
      <th scope="col">
        <div class="cds--table-header-label cds--table-header-label">
          Project Name
        </div>
      </th>
      <th scope="col">
        <div class="cds--table-header-label cds--table-header-label">
          Description
        </div>
      </th>
      <th scope="col">
        <div class="cds--table-header-label cds--table-header-label">
          Tags / Languages
        </div>
      </th>
    </tr>
  </thead>

  <tbody>
    {#each data as { name, description, url, languages_key, tags }}
      <tr>
        <td> <a href={url} target="_blank"> {name} </a> </td>
        <td>{description}</td>
        <td> {[...(tags || []), ...(languages_key || [])].join(", ")} </td>
      </tr>
    {/each}
  </tbody>
</table>
