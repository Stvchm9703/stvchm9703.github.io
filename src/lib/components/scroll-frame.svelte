<script lang="ts">
  // base on https://olivier3lanc.github.io/Scroll-Frames/

  import { onMount } from "svelte";
  interface Props {
    [key: string]: any
  }

  let { ...props }: Props = $props();

  // Helper function to clamp a number between a min and max
  const clamp = (num, min, max) => Math.min(Math.max(num, min), max);

  const prefetchFrame = 5,
    prefetchFrameOffset = 10;

  // Default configuration for the background style
  const defaultConfig = {
    style: {
      backgroundRepeat: "no-repeat",
      backgroundSize: "cover",
      backgroundPosition: "center",
    },
  };

  // Current state of the component
  let currentState = {
    ready: false,
    transfer: false,
    backgroundImage: "",
    backgroundSizes: "",
  };

  // Import all .webp files from the assets directory
  const frameMap = import.meta.glob("$assets/series/index-page/w1200/*.webp", {
    eager: true,
    as: "url",
  });

  // Create a list of frames with their URLs and IDs
  const frameList = Object.values(frameMap).map((url, index) => ({
    url,
    frameId: index,
  }));

  let _prefetchFrame = Array(prefetchFrame)
    .fill(0)
    .map((_, i) => i);
  const prefetchFrameList = frameList.filter(
    ({ frameId }) =>
      frameId % prefetchFrameOffset === 0 || _prefetchFrame.includes(frameId)
  );

  let frameIndex = $state(0);

  let elementRef = $state();

  // Function to build the background image and sizes
  const build = () => {
    currentState.backgroundImage = frameList
      .map((frame) => `url('${frame.url}')`)
      .join(", ");
    currentState.backgroundSizes = frameList
      .map((_, index) =>
        index === 0 ? defaultConfig.style.backgroundSize : "0%"
      )
      .join(", ");

    currentState.ready = true;
  };

  onMount(() => {
    const parentElement = document.querySelector("section#introduction");

    // Function to calculate the intersection of the element with the viewport
    const getIntersection = (el) => {
      let response = -1;
      if (
        typeof el == "object" &&
        typeof el.getBoundingClientRect == "function"
      ) {
        const box = el.getBoundingClientRect();
        const el_offset_top =
          box.top + window.scrollY - document.documentElement.clientTop;
        const el_height = el.clientHeight;
        const window_scroll_top =
          (document.documentElement && document.documentElement.scrollTop) ||
          document.body.scrollTop;
        const window_height = window.document.documentElement.clientHeight;

        response =
          (window_scroll_top - el_offset_top) /
          el_height /
          ((el_height - window_height) / el_height);
      }

      return clamp(response, 0, 1);
    };

    // Function to request a new animation frame
    const RAF = () => window.requestAnimationFrame(frame);

    // Function to update the background sizes based on the intersection
    const frame = () => {
      if (currentState.ready) {
        const intersection = getIntersection(parentElement);
        if (intersection >= 0 && intersection <= 1) {
          frameIndex = clamp(
            Math.floor(intersection * frameList.length),
            1,
            frameList.length
          );

          currentState.backgroundSizes = frameList
            .map((_, index) =>
              index + 1 === frameIndex
                ? defaultConfig.style.backgroundSize
                : "0%"
            )
            .join(", ");
        }
      }
    };

    build();
    frame();
    window.addEventListener("scroll", RAF);
  });

  // Reactive statement to update the style of the div

  // $: styleGetter = () =>
  //   `background-image: ${currentState.backgroundImage}; background-size: ${currentState.backgroundSizes}; background-position: center; background-repeat: no-repeat; will-change: background-size;`;
</script>

<svelte:head>
  <!-- {#each frameList as item}
<link rel="preload" href={item.url} as="image" data-frame-id={item.frameId} />   
{/each} -->
  {#each prefetchFrameList as item}
    <link
      rel="prefetch"
      href={item.url}
      as="image"
      data-frame-id={item.frameId}
    />
  {/each}
</svelte:head>

<div
  bind:this={elementRef}
  class={props.class}
  data-frame-current={frameIndex}
>
  <!-- <div class="absolute inset-0"> -->
  {#each frameList as { url, frameId }}
    <img
      class="absolute inset-0 object-cover h-full w-full {frameIndex ===
      frameId + 1
        ? 'opacity-100'
        : 'opacity-0'}"
      loading={_prefetchFrame.includes(frameId) ? "eager" : "lazy"}
      src={url}
      alt="scroll-framep; frame-{frameId}"
      data-frame-id={frameId}
    />
  {/each}
  <!-- </div> -->
</div>
