<script lang="ts">
  import { createBubbler } from "svelte/legacy";

  const bubble = createBubbler();

  interface Props {
    // export let isShow = false;
    isHoverOn?: string;
    sectionName?: string;
    sectionId?: string;
    href?: any;
  }

  let {
    isHoverOn = "",
    sectionName = "",
    sectionId = "",
    href = "/#" + sectionId,
  }: Props = $props();
</script>

<a
  {href}
  class="flex items-center py-1 relative"
  onmouseenter={bubble("mouseenter")}
  aria-label="menuitem"
>
  <i
    class="dot transition-all w-1 h-1 m-5 block bg-white z-1 {isHoverOn ===
    sectionId
      ? 'w-8'
      : '-'}"
  ></i>
</a>
