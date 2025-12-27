<script>
    const { active, label } = props();
</script>

<button
    class="h-8 rounded-sm bg-transparent py-2 px-4 data-[state=active]:shadow-mini dark:data-[state=active]:bg-muted data-[state=active]:bg-white"
    role="tab"
    data-state={active ? "active" : "inactive"}
    data-value="outputs"
    data-orientation="horizontal"
    aria-selected={active ? "true" : "false"}
    data-tabs-trigger
    tabindex="0"
    type="button"
    aria-controls=""
    bind:active
    {onclick}
>
    {label}
</button>
