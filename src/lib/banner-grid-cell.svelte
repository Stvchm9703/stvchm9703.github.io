<script>
    import { intersect } from "@svelte-put/intersect";
    // import { derived } from "svelte/store";

    // export let align = "left";
    // export let id = "";
    // export let row = 1;
    // export let isTitle = false;

    let {
        align = "left",
        id = "",
        row = 1,
        isTitle = false,
        ...props
    } = $props();

    let isIntersecting = $state(false);

    const toStyle = (styleObject) => {
        return Object.entries(styleObject)
            .map(([key, value]) => `${key}: ${value};`)
            .join(" ");
    };

    const onIntersect = (event) => {
        const { entries } = event.detail;
        const entry = entries[0];
        isIntersecting = entry.isIntersecting;
    };
    // $: rowClass = () => `grid-row-start-${row} grid-row-end-${row + 1}`;
    // $: alignClass = () =>
    // align === "left"
    //     ? " justify-self-end lg:justify-self-start 2xl:(grid-col-start-1 grid-col-end-2) "
    //     : align === "right"
    //       ? " justify-self-start lg:justify-self-end 2xl:(grid-col-start-3 grid-col-end-4) "
    //       : "";

    let alignClass = () =>
        align === "left"
            ? " justify-self-end lg:justify-self-start 2xl:(grid-col-start-1 grid-col-end-2) "
            : align === "right"
              ? " justify-self-start lg:justify-self-end 2xl:(grid-col-start-3 grid-col-end-4) "
              : "";

    // $: intersectingClass = () =>
    //     isIntersecting
    //         ? "translate-x-0 translate-y-0  opacity-100"
    //         : (align === "left"
    //               ? "translate-x--10rem"
    //               : align === "right"
    //                 ? "translate-x-10rem"
    //                 : "translate-y-10rem") + " opacity-0";

    let intersectingClass = $derived(
        isIntersecting
            ? "translate-x-0 translate-y-0  opacity-100"
            : (align === "left"
                  ? "translate-x--10rem"
                  : align === "right"
                    ? "translate-x-10rem"
                    : "translate-y-10rem") + " opacity-0",
    );
</script>

<div
    {id}
    class={"block lt-lg:w-full px-6 py-3 lg:(px-6 py-4)  rounded-lg transition transform  place-self-center  xl:f-my-16-56 overflow-hidden " +
        [
            !isTitle
                ? "backdrop-filter backdrop-blur-xl backdrop-brightness-80"
                : "",
            alignClass(),
            intersectingClass,
            props.class,
        ].join(" ")}
    style={toStyle({
        "grid-row-start": row,
        "grid-row-end": row + 1,
    })}
    use:intersect={{ threshold: 0.1 }}
    onintersectonce={onIntersect}
>
    {@render props.children()}
</div>
