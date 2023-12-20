<script>
  export let label = "Hello World";
  export let icon = null;
</script>

<div class="flex justify-start items-center bg-light-200 gap-3 py-3 px-4">
  <i class="w-6 h-6 border-dark700 border-5 border-solid rounded-full block bg-white"> </i>
  <div class="flex-1 flex justify-start items-center gap-3">
    {#if icon !== null}
      <img
        class="w-6 h-6 object-contain bg-white rounded-full"
        src={icon ? icon : null}
        alt="arrow-right"
      />
    {:else}
      <i class="w-6 h-6"></i>
    {/if}
    <span class="text-dark200">{label}</span>
  </div>
</div>
