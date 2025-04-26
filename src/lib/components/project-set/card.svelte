<script>
    let { item } = $props();
</script>

<a
    class="card rounded-lg p-4 backdrop-filter backdrop-blur-xl backdrop-brightness-80 text-white border-4 border-transparent hover:border-emerald transition-all"
    href={item.url}
    target="_blank"
>
    <div class="items">
        <div class="">
            <span class="text-lg font-bold line-clamp-2 break-words text-wrap"
                >{item.name}</span
            >
            <div class="mt-3 text-sm text-wrap line-clamp-4">
                {item.description}
            </div>
        </div>
    </div>
</a>
