<script>
  export let alt = "";
  export let height = 0; // needed to reduce CLS
  export let width = 0; // needed to reduce CLS
  export let src = "";
  // export let sources = [];
  export let maxWidth = "1280px";
  $: sizes = `(max-width: ${maxWidth}) 100vw, ${maxWidth})`;
  let searchSet = new URLSearchParams();
  let placeholder = "";
  $: {
    if (width && width != 0) searchSet.set("w", width);
    if (height && height != 0) searchSet.set("w", height);
    searchSet.set("webp", true);
    // placeholder =
    //   src && src != ""
    //     ? (src + "?" + searchSet.toString())
    //     : "";

    // if (src && src !== "") {
    //   import(src + "?" + searchSet.toString()).then((result) => {
    //     console.log(result)
    //     // placeholder = result;
    //   });
    // }

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

<picture class="object-contain inset-0">
  <img
    class="h-auto w-full"
    {alt}
    {importance}
    {loading}
    decoding="async"
    {width}
    height={height && height !== 0 ? height : null}
    data-src={src}
    type="image/webp"
    src={placeholder}
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
