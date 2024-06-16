<script>
  import { onMount } from "svelte";

  import NavItem from "./item.svelte";

  // import { clickoutside } from "@svelte-put/clickoutside";
  // let isShow = false;
  let isHoverOn = "";
  const navItemClick = (name) => {
    isHoverOn = name;
  };

  export let activeId = "index";
  $: {
    isHoverOn = activeId;
    console.log({isHoverOn, activeId});
  };
</script>

<nav class="fixed top-0 w-full z-6000">
  <!-- Navigation items go here -->
  <section class="flex justify-center items-center h-16 p-2">
    <NavItem
      bind:isHoverOn
      sectionName="Index"
      sectionId="index"
      on:mouseenter={() => navItemClick("index")}
    />
    <NavItem
      bind:isHoverOn
      sectionName="Introduction"
      sectionId="introduction"
      on:mouseenter={() => navItemClick("introduction")}
    />
    <NavItem
      bind:isHoverOn
      sectionName="Projects"
      sectionId="project"
      on:mouseenter={() => navItemClick("project")}
    />
    <NavItem
      bind:isHoverOn
      sectionName="Skill"
      sectionId="skill"
      on:mouseenter={() => navItemClick("skill")}
    />
  </section>
</nav>
