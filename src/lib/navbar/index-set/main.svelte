<script>
  import { onDestroy, onMount } from "svelte";
  let showDropdown = "";
  let isShow = false;
  let isHoverOn = "";
  let isActiveTab = "";

  let aHrefLink = {};
  const onScrollHandle = (event) => {
    // console.log(self);
  };
  onMount(() => {
    onScrollHandle();
    window.addEventListener("scroll", onScrollHandle);
  });
  onDestroy(() => {
    window.removeEventListener("scroll", onScrollHandle);
  });
</script>

<nav class=" fixed top-0 bottom-0 z-8000">
  <div
    class={"absolute  bg-dark400/15 inset-y-0 backdrop-filter  backdrop-blur-lg " +
      (isShow ? "w-45vw" : "w-0") +
      " transition-all duration-500"}
    on:mouseout={() => (isShow = false)}
  />

  <section class="absolute left-6 top-50% w-auto transform -translate-y-50%">
    <ul class="flex flex-col gap-1 w-auto min-w-0">
      <li
        class="flex items-center py-1 relative"
        on:mouseenter={() => ((isShow = true), (isHoverOn = "index"))}
      >
        <div
          class={"absolute h-100% bg-dark-800 z-0 transition-all duration-350 ease-in-out  " +
            (isHoverOn === "index" && isShow ? "w-100%" : "w-0")}
        />
        <i class="dot w-1 h-1 m-5 block bg-white z-1" />
        <a
          href="/#index"
          class={" text-white text-4xl leading-loose whitespace-nowrap block z-1 overflow-hidden transition-all duration-350 ease-in-out delay-250 " +
            (isShow ? "w-full mx-3" : "w-0 mx-0")}>Index</a
        >
      </li>
      <li
        class="flex items-center py-1 relative"
        on:mouseenter={() => ((isShow = true), (isHoverOn = "introduction"))}
      >
        <div
          class={"absolute h-100% bg-dark-800 z-0 transition-all duration-350 ease-in-out  " +
            (isHoverOn === "introduction" && isShow ? "w-100%" : "w-0")}
        />
        <i class="dot w-1 h-1 m-5 block bg-white z-1" />
        <a
          href="/#introduction"
          class={"text-white text-4xl leading-loose whitespace-nowrap block z-1 overflow-hidden transition-all duration-350 ease-in-out delay-250 " +
            (isShow ? "w-full mx-3 " : "w-0 mx-0 ")}>Introduction</a
        >
        <!-- <span class="text-white text-4xl leading-loose whitespace-nowrap">Introduction</span> -->
      </li>
      <li
        class="flex items-center py-1 relative"
        on:mouseenter={() => ((isShow = true), (isHoverOn = "project"))}
      >
        <div
          class={"absolute h-100% bg-dark-800 z-0 transition-all duration-350 ease-in-out  " +
            (isHoverOn === "project" && isShow ? "w-100%" : "w-0")}
        />
        <i class="dot w-1 h-1 m-5 block bg-white z-1" />
        <a
          href="/#project"
          class={"text-white text-4xl leading-loose whitespace-nowrap block z-1 overflow-hidden transition-all duration-350 ease-in-out delay-250 " +
            (isShow ? "w-full mx-3" : "w-0 mx-0")}>Project / Work Expiences</a
        >
        <!-- <span class="text-white text-4xl leading-loose whitespace-nowrap">Project / Work Expiences</span> -->
      </li>
      <li
        class="flex items-center py-1 relative"
        on:mouseenter={() => ((isShow = true), (isHoverOn = "skill"))}
      >
        <div
          class={"absolute h-100% bg-dark-800 z-0 transition-all duration-350 ease-in-out  " +
            (isHoverOn === "skill" && isShow ? "w-100%" : "w-0")}
        />
        <i class="dot w-1 h-1 m-5 block bg-white z-1" />
        <a
          href="/#skill"
          class={" text-white text-4xl leading-loose whitespace-nowrap block z-1 overflow-hidden transition-all duration-350 ease-in-out delay-250 " +
            (isShow ? "w-full mx-3" : "w-0 mx-0")}>Skill</a
        >
        <!-- <span class="text-white text-4xl leading-loose whitespace-nowrap">Skill</span> -->
      </li>
    </ul>
    <hr class="mx-2 my-4 w-8" />
    <ul class="flex flex-col gap-1">
      <li
        class="flex items-center py-1 relative"
        on:mouseenter={() => ((isShow = true), (isHoverOn = "blog"))}
      >
        <div
          class={"absolute h-100% bg-dark-800 z-0 transition-all duration-350 ease-in-out  " +
            (isHoverOn === "blog" && isShow ? "w-100%" : "w-0")}
        />
        <i class="dot w-1 h-1 m-5 block bg-white z-1" />
        <a
          class={" text-white text-4xl leading-loose whitespace-nowrap block z-1 overflow-hidden transition-all duration-350 ease-in-out delay-250 " +
            (isShow ? "w-full mx-3" : "w-0 mx-0")}
          href="/blog">Blog</a
        >
        <ul />
      </li>
      <li
        class="flex items-center py-1 relative"
        on:mouseenter={() => ((isShow = true), (isHoverOn = "contact"))}
      >
        <div
          class={"absolute h-100% bg-dark-800 z-0 transition-all duration-350 ease-in-out  " +
            (isHoverOn === "contact" && isShow ? "w-100%" : "w-0")}
        />
        <i class="dot w-1 h-1 m-5 block bg-white z-1" />
        <a
          href="/#contact"
          class={" text-white text-4xl leading-loose whitespace-nowrap block z-1 overflow-hidden transition-all duration-350 ease-in-out delay-250 " +
            (isShow ? "w-full mx-3" : "w-0 mx-0")}>Contact</a
        >
      </li>
    </ul>
  </section>
</nav>
