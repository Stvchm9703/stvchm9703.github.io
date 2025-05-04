<script>
    import "uno.css";
    import { buttonVariants } from "$lib/components/ui/button";
    import { Separator } from "$lib/components/ui/separator";
    import Headline from "$lib/components/post-layout/headline.svelte";
    import PostContainer from "$lib/components/post-layout/post-list-container.svelte";
    import PostCard from "$lib/components/post-layout/card";
    import { cn, displayDate, pathResolver } from "$lib/utils";
    import { page } from "$app/state";
    import { MetaTags } from "svelte-meta-tags";
    const { series: rawSeries, meta } = page.data;
    const series = rawSeries.filter(
        (elm) => elm.resultList.length > 0 && elm.id !== "latestUpdated",
    );

    const latest_update = series.find((elm) => elm.id !== "latestUpdated");

    // console.log(meta);
</script>

<MetaTags {...meta} />

<Headline post={latest_update.resultList[0]} />

<Separator class="w-full" />

<section
    class={cn([
        "w-full ",
        "lg:grid grid-cols-3 grid-flow-dense ",
        "flex flex-col justify-between",
    ])}
>
    {#each series as serie, serie_key}
        <PostContainer
            key={serie.id}
            title={serie.name}
            subtitle={serie.description}
            url={`/posts/series/${pathResolver(serie.name)}_${serie._sid}`}
            class={cn([
                "px-6 py-4",
                serie_key % 3 == 2 ? "" : "border-r-solid b-r-1 b-solid",
            ])}
        >
            {#each serie.resultList as post}
                <PostCard
                    id={post.id}
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
<section class="w-full flex justify-center">
    <a
        class={cn([
            buttonVariants.variants.variant.link,
            buttonVariants.variants.size.lg,
            "leading-12",
        ])}
        href="/posts/series"
    >
        Load More
    </a>
</section>
