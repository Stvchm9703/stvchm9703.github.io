<script>
  import { onMount } from "svelte";

  export let id = "";
  export let as = "div";
  export let href = "";
  export let color = "";
  export let stacked = false;
  export let iconSrc = "";
  // export let position = "static";
  export let pos = { x: 0, y: 0 };
  export let title = "",
    subtitle = "",
    label;
  export let active = false;
  export let actions = false;
  export let onActionClick = () => {};

  let namespace = `cds--cc--card-node`;
  let component = "div";

  let unitWidth = 200,
    unitHeight = 64;

  onMount(() => {
    if (href) {
      component = "a";
    } else {
      component = as;
    }
  });

  $: classname = [
    namespace,
    stacked ? `${namespace}--stacked` : "",
    `${namespace}--${component}`,
    $$props.class,
  ].join(" ");

  $: style = [`border-color: ${color}`, `position: static`, $$props.style].join(
    ";"
  );
</script>

<foreignObject
  id="skill-tree-card-{id}"
  data-card-id={id}
  transform="translate({pos.x}, {pos.y})"
  height={unitHeight}
  width={unitWidth}
  style="overflow: visible;"
>
  <svelte:element
    this={component}
    class={classname}
    {style}
    on:click
    {active}
    tabindex="0"
    {href}
  >
    {#if iconSrc && iconSrc !== ""}
      <div class="cds--cc--card-node__column !px-0">
        <img class="cds--cc--card-node__icon block h-7 w-7" src={iconSrc} />
      </div>
    {/if}

    <div class="cds--cc--card-node__column">
      <slot>
        <div class="cds--cc--card-node__title">{title}</div>
        <div class="cds--cc--card-node__subtitle">{subtitle}</div>
        {#if label}
          <div class="cds--cc--card-node__label">{label}</div>
        {/if}
      </slot>
    </div>
    {#if actions}
      <div
        class="cds--cc--card-node__actions cds--cc--card-node__column--farside"
        on:click|stopPropagation={onActionClick}
      >
        <svg
          focusable="false"
          preserveAspectRatio="xMidYMid meet"
          xmlns="http://www.w3.org/2000/svg"
          fill="currentColor"
          aria-hidden="true"
          width="16"
          height="16"
          viewBox="0 0 16 16"
          ><path d="M8 11L3 6 3.7 5.3 8 9.6 12.3 5.3 13 6z"></path></svg
        >
      </div>
    {/if}
  </svelte:element>
</foreignObject>
