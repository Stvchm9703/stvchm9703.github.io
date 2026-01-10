<script lang="ts">
  import {
    Sheet,
    SheetContent,
    SheetTrigger,
    SheetOverlay,
  } from "$lib/components/ui/sheet/index.js";
  import { cn } from "$lib/utils";
  import { onMount } from "svelte";
  import NavItem from "./item.svelte";
  let isShow = $state(false);
  let activeTab = $state("index");

  const handleNavItemClick = (sectionId: string) => {
    activeTab = sectionId;
    setTimeout(() => {
      isShow = false;
    }, 300);
  };

  onMount(() => {
    window.addEventListener("hashchange", () => {
      const hash = window.location.hash.replace("#", "");
      activeTab = hash || "index";
    });
  });

</script>

<Sheet bind:open={isShow}>
  <SheetTrigger
    class={cn(
      "fixed w-12 left-0 top-50% -translate-y-50% h-auto min-h-40 flex flex-col z-50 bg-dark-400/40 hover:bg-dark-400/80 transition-all",
      "rounded-r-sm"
    )}
  >
    <span class="sr-only">Toggle menu</span>
    <div class="flex flex-col items-center justify-center gap-2">
      <i
        class={cn(
          "dot w-1 h-1 my-3 block bg-white z-1 transition-all duration-150 ease-in-out",
          activeTab === "index" && "w-4"
        )}
      ></i>
      <i
        class={cn(
          "dot w-1 h-1 my-3 block bg-white z-1 transition-all duration-150 ease-in-out",
          activeTab === "introduction" && "w-4"
        )}
      ></i>
      <i
        class={cn(
          "dot w-1 h-1 my-3 block bg-white z-1 transition-all duration-150 ease-in-out",
          activeTab === "project" && "w-4"
        )}
      ></i>
      <i
        class={cn(
          "dot w-1 h-1 my-3 block bg-white z-1 transition-all duration-150 ease-in-out",
          activeTab === "skill" && "w-4"
        )}
      ></i>
      <hr class="border-white border-top-1 border-solid w-full" />
      <i class={cn("dot w-1 h-1 my-3 block bg-white z-1")}></i>
      <i class={cn("dot w-1 h-1 my-3 block bg-white z-1")}></i>
    </div>
  </SheetTrigger>

  <SheetOverlay
    class="transition-all duration-500 bg-dark-400/15 backdrop-filter backdrop-blur-xl backdrop-brightness-80"
  />
  <SheetContent
    side="left"
    class="w-full xl:w-3/4  p-0 transition-all duration-300 border-none inset-y-0 "
  >
    <nav class="flex flex-col gap-4 p-4 h-full flex-1 justify-center">
      <div class="flex flex-col gap-1 mb-1">
        <NavItem
          sectionName="Index"
          sectionId="index"
          onclick={() => handleNavItemClick("index")}
          {activeTab}
        />
        <NavItem
          sectionName="Introduction"
          sectionId="introduction"
          onclick={() => handleNavItemClick("introduction")}
          {activeTab}
        />
        <NavItem
          sectionName="Projects"
          sectionId="project"
          onclick={() => handleNavItemClick("project")}
          {activeTab}
        />
        <NavItem
          sectionName="Skill"
          sectionId="skill"
          onclick={() => handleNavItemClick("skill")}
          {activeTab}
        />
      </div>
      <hr
        class={"mx-2 my-4 transition-all border-white border-top-1 border-solid w-full"}
      />
      <div class="flex flex-col gap-1 mt-1">
        <NavItem sectionName="Blog Post" sectionId="blog" href="/posts" />
        <NavItem sectionName="Contact" sectionId="contact" />
      </div>
    </nav>
  </SheetContent>
</Sheet>
