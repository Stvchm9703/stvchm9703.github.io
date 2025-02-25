<script>
  // export let iconSrc = "";
  // export let colorCode = "";
  // export let title = "";

  const { colorCode, iconSrc, title, ...props } = $props();
  let styleObj = $derived(`border-color : ${colorCode}`);

  let isActive = $state(false);
</script>

<div
  class="transition-all p-4 overflow-hidden b-2 b-solid bg-coolgray-6 bg-op-30 rounded-full hover:rounded-lg {props.class} flex flex-col items-center justify-center"
  style={styleObj}
  on:mouseover={() => (isActive = true)}
  on:mouseout={() => (isActive = false)}
>
  <img class="w-20 h-20" src={iconSrc} alt={title} />
  {#if isActive}
    <span class="w-full text-center text-white text-lg">
      {title}
    </span>
  {/if}
</div>
