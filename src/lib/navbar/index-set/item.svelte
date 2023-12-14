<script>
  export let isShow = false;
  export let isHoverOn = "";
  export let sectionName = "";
  export let sectionId = "";
  export let href = "/#" + sectionId;
</script>

<a {href} class="flex items-center py-1 relative" on:mouseenter>
  <div
    class={"absolute h-100%  bg-gray700 z-0 transition-all duration-350 ease-in-out  " +
      (isHoverOn === sectionId && isShow ? "w-full" : "w-0")}
  />
  <i class="dot w-1 h-1 m-5 block bg-white z-1" />
  <span
    class={" text-white text-4xl leading-loose whitespace-nowrap block z-1 overflow-hidden transition-all duration-350 ease-in-out delay-250 " +
      (isShow ? "w-auto mx-3 opacity-100 " : "w-0 mx-0 opacity-0")}
    >{sectionName}</span
  >
</a>
