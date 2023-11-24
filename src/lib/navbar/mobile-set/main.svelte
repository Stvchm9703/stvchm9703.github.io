<script>
  import NavItem from "./item.svelte";
  import { clickoutside } from "@svelte-put/clickoutside";
  let isShow = false;
  let isHoverOn = "";
  const navItemClick = (name) => {
    setTimeout(() => {
      isShow = false;
    }, 200)
    isHoverOn = name;
  };

  $: navMenuClass =
    "absolute top-16 w-full b-solid b-x-none b-b-none  b-1 bg-dark400/65  backdrop-blur-lg transition-all duration-500 " +
    (isShow ? "b-white" : "b-transparent");
</script>

<nav
  class="fixed top-0 w-full backdrop-blur-lg bg-dark400/35 text-white z-8000"
>
  <!-- Navigation items go here -->
  <div class="flex justify-between items-center h-16 p-2">
    <div class="">Block</div>
    <div class="flex flex-1 justify-end items-center">
      <button
        class="block rounded p-1 w-8 h-8 bg-transparent transition-all hover:(bg-gray700/25)"
        on:click={() => (isShow = !isShow)}
      >
        <i class="i-carbon-menu text-white w-full h-full block" />
      </button>
    </div>
  </div>
  <div class={navMenuClass}>
      <section class={"flex flex-col overflow-hidden max-h-[calc(100vw-4rem)] transition-all " + (isShow ? 'h-auto' : 'h-0')}>
        <ul class="flex flex-col gap-1 mb-1">
          <NavItem
            bind:isShow
            bind:isHoverOn
            sectionName="Index"
            sectionId="index"
            on:mouseenter={() => navItemClick("index")}
          />

          <NavItem
            bind:isShow
            bind:isHoverOn
            sectionName="Introduction"
            sectionId="introduction"
            on:mouseenter={() => navItemClick("introduction")}
          />
          <NavItem
            bind:isShow
            bind:isHoverOn
            sectionName="Projects"
            sectionId="project"
            on:mouseenter={() => navItemClick("project")}
          />
          <NavItem
            bind:isShow
            bind:isHoverOn
            sectionName="Skill"
            sectionId="skill"
            on:mouseenter={() => navItemClick("skill")}
          />
        </ul>
      </section>
  </div>
</nav>
