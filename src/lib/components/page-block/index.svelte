<script>
    import BannerGridCell from "$components/banner-grid-cell.svelte";
    import BaseImage_4 from "$assets/img/coffee.jpg?format=webp&w=700;1200&q=85&as=metadata";

    import KeyboardImage from "$assets/img/keyboard.jpg?format=webp&q=85";
    import GameAnimation from "$assets/animation/alicetaria-walking.webm?url&format=webm";
</script>

<h1
    class="grid-row-start-11 grid-col-start-2 grid-col-end-2 text-center text-light-50 text-5xl mt-6 mb-3"
>
    Hobby
</h1>
<BannerGridCell align="left" row={12}>
    <h2 class="text-light-50 text-3xl">Drawing</h2>
    <div class="text-light-50 text-lg">
        just a little bit, not much, but I like it
        <div class="grid w-24 h-24"></div>
    </div>
</BannerGridCell>
<BannerGridCell align="right" row={13}>
    <h2 class="text-light-50 text-3xl">Program making</h2>
    <div class="text-light-50 text-lg">
        make some small util tools for myself, to make my life easier; but not a
        big fans of code leeting
        <div class="grid w-24 h-24"></div>
    </div>
</BannerGridCell>
<BannerGridCell align="left" row={14}>
    <h2 class="text-light-50 text-3xl">Game making</h2>
    <div class="text-light-50 text-lg">
        make some small games, but not a big fans of game playing
    </div>
</BannerGridCell>
<BannerGridCell align="right" row={14} class="!p-0 !backdrop-filter-none">
    <video
        class="aspect-square w-full block max-h-4xl"
        src={GameAnimation}
        preload="none"
        alt="animation"
        loop="true"
        autoplay="true"
    >
        <track kind="captions" />
    </video>
</BannerGridCell>
<BannerGridCell align="right" row={16}>
    <h2 class="text-light-50 text-3xl">Keyboard</h2>
    <p class="text-light-50 text-lg">
        nice to type on, nice to look at. speed up my typing speed; also, make
        for cool stuff
    </p>
    <img
        class="mt-5 w-full min-h-24 object-contain aspect-ratio-video"
        src={KeyboardImage}
        alt=""
    />
</BannerGridCell>

<BannerGridCell align="left" row={17}>
    <h2 class="text-light-50 text-3xl">Coffee(?)</h2>
    <div class="text-light-50 text-lg">coffee energy, enpower me everyday</div>
    <!-- <img class="mt-5 w-full min-h-24 object-contain" src={BaseImage_2} alt="" /> -->
    <ImageSet
        class="mt-5 w-full min-h-24 object-contain aspect-ratio-video rounded"
        srcList={BaseImage_4}
        alt="coffee image"
    />
</BannerGridCell>
