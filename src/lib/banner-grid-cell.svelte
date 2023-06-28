<script>
  export let align = "left";
</script>

<div
  class={"block px-6 py-4 rounded-lg" +
    "backdrop-filter backdrop-blur-xl backdrop-brightness-80 " +
    (align === "left"
      ? "justify-self-end lg:justify-self-start grid-col-start-0 grid-col-end-1"
      : align === "right"
      ? "justify-self-start lg:justify-self-end grid-col-start-2 grid-col-end-3"
      : "") +
    " " +
    $$props.class}
>
  <slot />
</div>
