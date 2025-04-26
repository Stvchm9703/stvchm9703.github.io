<script>
    import { cn } from "$lib/utils.ts";
    export let direction = "left";
    export let pauseOnHover = false;
    export let reverse = false;
    export let fade = false;
    export let innerClassName = "";
    export let numberOfCopies = 2;
</script>

<div
    class={cn(
        `group flex gap-[1rem] overflow-hidden py-2 ${$$restProps.class}`,
        {
            "flex-row": direction === "left",
            "flex-col": direction !== "left",
        },
    )}
    style={`mask-image: ${
        fade
            ? `linear-gradient(${
                  direction === "left" ? "to right" : "to bottom"
              }, transparent 0%, rgba(0, 0, 0, 1.0) 10%, rgba(0, 0, 0, 1.0) 90%, transparent 100%)`
            : "none"
    };
	  -webkit-mask-image: ${
          fade
              ? `linear-gradient(${
                    direction === "left" ? "to right" : "to bottom"
                }, transparent 0%, rgba(0, 0, 0, 1.0) 10%, rgba(0, 0, 0, 1.0) 90%, transparent 100%)`
              : "none"
      };
	  `}
>
    {#each Array(numberOfCopies).fill(0) as _, i (i)}
        <div
            class={cn(
                "flex justify-around gap-[1rem] [--gap:1rem] shrink-0",
                direction === "left"
                    ? "animate-marquee-left flex-row"
                    : "animate-marquee-up flex-col",
                pauseOnHover && "group-hover:[animation-play-state:paused]",
                reverse && "direction-reverse",
                innerClassName,
            )}
        >
            <slot />
        </div>
    {/each}
</div>
