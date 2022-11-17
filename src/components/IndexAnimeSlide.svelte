<script lang="ts">
  interface IAnimationFrame {
    scenceId: string
    startFrame: number
    endFrame: number
  }
  import {onMount} from 'svelte'
  let isAbleAnimation: boolean = false
  export let animationFrameSet: IAnimationFrame = {
    scenceId: '',
    startFrame: 0,
    endFrame: 0,
  }
  export let forceAnimation: boolean = false;
  onMount(() => {
    isAbleAnimation = forceAnimation || window.matchMedia(`(prefers-reduced-motion: reduce)`).matches === true
  })

  const onScrollAction = () => {

  }
</script>

{#if forceAnimation}
  <div class="animation-container static block w-screen min-h-screen" on:scroll={onScrollAction} style:height="{`calc(100vh + ${(animationFrameSet.endFrame - animationFrameSet.startFrame) * 50}px)`}">
    <div class="__front-layer z-10 relative">
      <h1 class="text-title text-h1">hi this</h1>
    </div>
    <div class="__background strcky  h-screen w-screen top-0 bg-red-700"></div>
    
  </div>
{:else}
  <div class="animation-container--empty  static block w-screen min-h-screen">not y</div>
{/if}
