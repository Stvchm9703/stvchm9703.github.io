<script>
  // export let isShow = false;
  export let isHoverOn = "";
  export let sectionName = "";
  export let sectionId = "";
  export let href = "/#" + sectionId;
</script>

<a {href} class="flex items-center py-1 relative" on:mouseenter>
  <i
    class="dot transition-all w-1 h-1 m-5 block bg-white z-1 {isHoverOn === sectionId
      ? 'w-8'
      : '-'}"
  />
</a>
