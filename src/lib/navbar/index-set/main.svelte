<script>
  import NavItem from "./item.svelte";
  import { clickoutside } from "@svelte-put/clickoutside";
  let isShow = false;
  let isHoverOn = "";
  const navItemClick = (name) => {
    isShow = true;
    isHoverOn = name;
  };
</script>

<nav class=" fixed top-0 bottom-0 z-8000">
  <div
    class={"absolute  bg-dark400/15 inset-y-0 backdrop-filter  backdrop-blur-lg transition-all duration-500 " +
      (isShow ? "w-45vw" : "w-0")}
    use:clickoutside
    on:clickoutside={() => (isShow = false)}
    on:mouseout={() => (isShow = false)}
  />

  <section
    class={"absolute left-6 top-50% transform -translate-y-50% " +
      (isShow ? " w-[calc(45vw-1.5rem)]" : " w-12")}
  >
    <div class="flex flex-col gap-1 mb-1">
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
    </div>
    <hr class={"mx-2 my-4 transition-all " + (isShow ? "w-full" : "w-8")} />
    <div class="flex flex-col gap-1 mt-1">
      <NavItem
        bind:isShow
        bind:isHoverOn
        sectionName="Blog Post"
        sectionId="blog"
        href="/posts"
        on:mouseenter={() => navItemClick("blog")}
      />
      <NavItem
        bind:isShow
        bind:isHoverOn
        sectionName="Contact"
        sectionId="contact"
        on:mouseenter={() => navItemClick("contact")}
      />
    </div>
  </section>
</nav>
