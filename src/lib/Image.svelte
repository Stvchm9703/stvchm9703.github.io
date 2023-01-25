<script>
  export let alt = "";
  export let height = 0; // needed to reduce CLS
  export let width = 1280; // needed to reduce CLS
  export let src = "";
  export let sources = [];
  export let maxWidth = "1280px";
  $: sizes = `(max-width: ${maxWidth}) 100vw, ${maxWidth})`;
  let searchSet = new URLSearchParams();
  let placeholder = "";
  $: {
    if (width && width != 0) searchSet.set("w", width);
    if (height && height != 0) searchSet.set("w", height);
    placeholder = src && src != "" ? ( src + "?" + searchSet.toString()) : "";

    // console.log(import.meta.resolve( src + "?" + searchSet.toString()))
    // if (sources.length == 0) {
    //   sources = [
    //     {
    //       media: "",
    //       srcset: placeholder+ "&webp",
    //       type: "image/webp",
    //     },
    //   ];
    // }
  }

  export let importance = false;
  export let loading = "lazy";
  // export let className = "";
</script>

<picture class="object-center inset-0 absolute">
  <img
    class="h-auto"
    {alt}
    {importance}
    {loading}
    decoding="async"
    {width}
    height={height && height !== 0 ? height : null}
    data-src={src}
    type="image/webp"
    src={placeholder + "&webp"}
    srcset={placeholder}
  />
</picture>

<style>
  /* img {
    height: auto;
  } */
  img:not([src]):not([srcset]) {
    visibility: hidden;
  }
</style>
