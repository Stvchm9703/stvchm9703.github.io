<script>
    import "uno.css";
    import { Separator } from "$lib/components/ui/separator";
    import Headline from "$lib/components/post-layout/headline.svelte";
    import PostContainer from "$lib/components/post-layout/post-list-container.svelte";
    import PostCard from "$lib/components/post-layout/card";
    import { cn, displayDate } from "$lib/utils";

    import { page } from "$app/state";
    import { MetaTags } from "svelte-meta-tags";
    const { series: rawSeries, meta } = page.data;
    const series = rawSeries.filter(
        (elm) => elm.resultList.length > 0 && elm.id !== "latestUpdated",
    );

    const latest_update = rawSeries.find((elm) => elm.id !== "latestUpdated");
    // console.log(meta);
</script>

<MetaTags {...meta} />

<Headline post={latest_update.resultList[0]} />

<Separator class="w-full" />

<section class="w-full justify-between lg:grid grid-cols-3 flex flex-col">
    {#each series as serie, serie_key}
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
                    tags={post.tags}
                />
            {/each}
        </PostContainer>
    {/each}
</section>
