<script lang="ts">
  interface Props {
    header?: import('svelte').Snippet;
    children?: import('svelte').Snippet;
    modal?: import('svelte').Snippet;
    [key: string]: any
  }

  let { ...props }: Props = $props();
</script>

<div class="chart-holder cds--chart-holder {props.class} bg-gray-300">
  <div
    class="cds--cc--chart-wrapper cds--cc--layout-column"
    style="height: 100%; width: 100%;"
  >
    {@render props.header?.()}
    <div
      class="layout-child spacer cds--cc--spacer"
      style="height: 15px; width: 100%;"
    ></div>

    <!-- container-frame -->
    <div
      class="layout-child full-frame cds--cc--layout cds--cc--layout-column-reverse"
      style="height: 1345px; width: 100%;"
    >
      <div class="layout-child graph-frame w-full h-full">
        {@render props.children?.()}
      </div>
    </div>
  </div>
  {@render props.modal?.()}
</div>
