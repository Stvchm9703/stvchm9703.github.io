<script></script>

<div class="chart-holder cds--chart-holder {$$props.class} bg-gray-300">
  <div
    class="cds--cc--chart-wrapper cds--cc--layout-column"
    style="height: 100%; width: 100%;"
  >
    <slot name="header" />
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
        <slot />
      </div>
    </div>
  </div>
  <slot name="modal" />
</div>
