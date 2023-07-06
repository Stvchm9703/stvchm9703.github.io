<script>
  import { intersect } from "@svelte-put/intersect";
  export let align = "left";
  let isIntersecting = false;
  const onIntersect = (event) => {
    const { entries } = event.detail;
    const entry = entries[0];
    isIntersecting = entry.isIntersecting;
  };
</script>

<div
  class={"block px-6 py-4 rounded-lg transition transform  " +
    "backdrop-filter backdrop-blur-xl backdrop-brightness-80 " +
    (align === "left"
      ? "justify-self-end lg:justify-self-start grid-col-start-0 grid-col-end-1"
      : align === "right"
      ? "justify-self-start lg:justify-self-end grid-col-start-2 grid-col-end-3"
      : "") +
    " " +
    (isIntersecting
      ? "translate-x-0 opacity-100"
      : (align === "left"
          ? "translate-x--10rem"
          : align === "right"
          ? "translate-x-10rem"
          : "translate-y-10rem") + " opacity-0") +
    " " +
    $$props.class}
  use:intersect={{ threshold: 0.1 }}
  on:intersectonce={onIntersect}
>
  <slot />
</div>
