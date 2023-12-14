<script>
  let x = 0,
    y = 0 ,
    z = 1,
    isMove = false;

  $: position = `translate(${x}px, ${y}px)`;
  $: mapStyle = `transform: ${position} ;`;

  const handleMouseMove = (event) => {
    if (!isMove) return;
    console.log(event);
    x += event.movementX;
    y += event.movementY;
  };
  const handleMouseWheel = (event) => {
    // console.log(event);
    // z += event.deltaY * -0.01;
  };
</script>

<div
  class="overflow-hidden -z-1 absolute inset-0 bg-gray700 {$$props.class}"
  on:mousedown={()=> (isMove = true)}
  on:mouseup={()=> (isMove = false)}
  on:mousemove={handleMouseMove}
>
  <!-- map render -->
  <div class="h-500vh w-500vh bg-blue" style={mapStyle}>
    <div class="sample-map grid grid-cols-10 grid-rows-10 gap-4 relative w-full h-full">
      {#each Array(100) as _, i}
        <div class="bg-red-500 min-h-32 min-w-8"></div>
      {/each}
    </div>
  </div>
</div>
