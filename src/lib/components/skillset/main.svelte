<script>
    import Card from "./card.svelte";
    import { nodes } from "./data.js";
    import "./style.css";
    import Marquee from "./marguee.svelte";
</script>

<div class="w-full h-full">
    <!-- filter -->

    <div class="flex flex-wrap gap-8">
        <client>
            <Marquee fade={true} pauseOnHover={true}>
                {#each nodes.slice(0, nodes.length / 3) as skillItem}
                    <Card
                        iconSrc={skillItem.icon}
                        colorCode={skillItem.color}
                        title={skillItem.name}
                    />
                {/each}
            </Marquee>
            <Marquee fade={true} reverse={true} pauseOnHover={true}>
                {#each nodes.slice(nodes.length / 3, (nodes.length / 3) * 2) as skillItem}
                    <Card
                        iconSrc={skillItem.icon}
                        colorCode={skillItem.color}
                        title={skillItem.name}
                    />
                {/each}
            </Marquee>
            <Marquee fade={true} pauseOnHover={true}>
                {#each nodes.slice((nodes.length / 3) * 2) as skillItem}
                    <Card
                        iconSrc={skillItem.icon}
                        colorCode={skillItem.color}
                        title={skillItem.name}
                    />
                {/each}
            </Marquee>
        </client>
    </div>
</div>
