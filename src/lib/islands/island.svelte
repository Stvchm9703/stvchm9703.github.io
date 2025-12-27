<is-land on:visible>
    <div id="svelte-app-client"></div>
    <template data-island>
        <script type="module">
            import { mount } from "svelte";
            import App from "{{ component.clientJsUrl }}";

            mount(App, {
                target: document.getElementById("svelte-app-client"),

                // prop override
                props: {
                    name: "milky way galaxy",
                },
            });
        </script>
    </template>
</is-land>
