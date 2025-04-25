<script>
    import { Separator } from "$lib/components/ui/separator";
    import Headline from "$lib/post-layout/headline.svelte";
    import PostContainer from "$lib/post-layout/post-list-container.svelte";
    import PostCard from "$lib/post-layout/card";
    import { cn, displayDate } from "$lib/utils";

    import { page } from "$app/state";
    const { series: rawSeries } = page.data;
    const series = rawSeries.filter(
        (elm) => elm.resultList.length > 0 && elm.id !== "latestUpdated",
    );

    const latest_update = rawSeries.find((elm) => elm.id !== "latestUpdated");
</script>

<Headline post={latest_update.resultList[0]} />

<Separator class="w-full" />

<section class="w-full justify-between lg:grid grid-cols-3 flex flex-col">
    {#each series as serie, serie_key}
        <!-- {#if serie_key !== 0}
            <Separator orientation="vertical" />
        {/if} -->
        <PostContainer
            key={serie.id}
            serie={serie.id}
            title={serie.name}
            class={cn([
                "px-6 py-4",
                serie_key % 3 == 2 ? "" : "border-r-solid b-r-1 b-solid",
            ])}
        >
            {#each serie.resultList as post}
                <PostCard
                    key={post.id}
                    title={post.title}
                    content={post.snippet}
                    post_date={displayDate(post.publish_date)}
                    link={post.url}
                />
            {/each}
        </PostContainer>
    {/each}
</section>
