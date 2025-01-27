<script>
export let item = {}
</script>


<a class="card rounded-lg p-4 backdrop-filter backdrop-blur-xl backdrop-brightness-80 text-white border-4 border-transparent hover:border-emerald transition-all"
href={item.url}
target="_blank"
>
  <div class="flex items">
    <!-- <div class="flex-none w-12 h-12 rounded-full"></div> -->
    <div class="flex-grow ml-4">
      <span class="text-lg font-bold line-clamp-2 break-words text-wrap">{item.name}</span>
      <div class="mt-3 text-sm text-wrap line-clamp-4">{item.description}</div>
    </div>
  </div>

</a>