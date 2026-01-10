<script lang="ts" module>
  // This component no longer relies on isShow and isHoverOn props.
  export interface Props {
    sectionName?: string;
    sectionId?: string;
    href?: string;
    activeTab?: string;
    onclick?: () => void;
  }
</script>

<script lang="ts">
  import { cn } from "$lib/utils";
  let {
    sectionName = "",
    sectionId = "",
    href = "/#" + sectionId,
    activeTab = "",
    onclick = () => {},
  }: Props = $props();

  const isActive = () => activeTab === sectionId;
</script>

<a {href} class={cn("flex items-center py-1 relative group")} {onclick}>
  <div
    class={cn(
      "absolute h-100% bg-gray700 z-0 transition-all duration-350 ease-in-out ",
      "group-hover:w-full",
      isActive() ? "w-full" : "w-0"
    )}
  ></div>
  <i class={cn("dot w-1 h-1 m-5 block bg-white z-1", isActive() && "w-4")}></i>
  <span
    class={"text-white text-lg xl:text-4xl leading-loose whitespace-nowrap block z-1 overflow-hidden transition-all duration-350 ease-in-out delay-250 " +
      "group-hover:(w-auto mx-3 opacity-100) " +
      "mx-0"}>{sectionName}</span
  >
</a>
