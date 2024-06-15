<script>
  export let source = { x: 0, y: 0 };
  export let target = { x: 0, y: 0 };
  export let middleAxisAdjust = null;
  export let assignId = "";
  export let distance = 0;
  // export let lineStyle = ""; // dash-sm,  dash-md, dash-lg, dash-xl, tunnel, double
  export let variant; // default, highlight
  export let color;
  $: pathLine =
    "M" +
    [
      `${source.x},${source.y}`,
      middleAxisAdjust !== null
        ? `${(source.x + target.x) / 2 - middleAxisAdjust},${source.y}`
        : source.y !== target.y || source.x !== target.x
          ? `${(source.x + target.x) / 2},${source.y}`
          : "",
      middleAxisAdjust !== null
        ? `${(source.x + target.x) / 2 - middleAxisAdjust},${target.y}`
        : source.y !== target.y || source.x !== target.x
          ? `${(source.x + target.x) / 2},${target.y}`
          : "",
      `${target.x},${target.y}`,
    ]
      .filter((el) => el && el !== "")
      .join("L");
</script>

<g
  ibm-graph-edge
  class="cds--cc--edge {variant ? `cds--cc--edge--${variant}` : ''} "
  transform={$$props.transform ?? ""}
>
  <path d={pathLine} class="cds--cc--edge__container" />
  <path d={pathLine} class="cds--cc--edge__outer" />
  <path
    d={pathLine}
    class="cds--cc--edge__inner"
    marker-end="url(#undefined)"
    marker-start="url(#undefined)"
    style={color ? `stroke: ${color}` : ""}
  />
</g>
