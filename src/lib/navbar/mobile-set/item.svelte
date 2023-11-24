<script>
  export let isShow = false;
  export let isHoverOn = "";
  export let sectionName = "";
  export let sectionId = "";
  export let href = "/#" + sectionId;
</script>

<li class="flex items-center py-1 relative" on:mouseenter>
  <div
    class={"absolute h-100% bg-gradient-to-rb  from-gray700 via-blue900 to-cyan400 z-0 transition-all duration-350 ease-in-out  " +
      (isHoverOn === sectionId && isShow ? "w-full" : "w-0")}
  />
  <i class="dot w-1 h-1 m-5 block bg-white z-1" />
  <a
    href={href}
    class={" text-white text-lg leading-loose whitespace-nowrap block flex-1 w-full z-1 overflow-hidden transition-all duration-250 ease-in-out delay-150 " +
      (isShow ? "w-auto mx-3 opacity-100 " : "w-0 mx-0 opacity-0")}>{sectionName}</a
  >
</li>
