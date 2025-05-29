<script lang="ts">
    import { page } from "$app/stores";
    import PostCard from "$lib/components/post-layout/card";
    import { buttonVariants } from "$lib/components/ui/button";
    import { displayDate, cn } from "$lib/utils";
    import { MetaTags } from "svelte-meta-tags";
    import { kebabCase } from "lodash-es";

    const { meta, tag } = $page.data;
</script>

<MetaTags {...meta} />

<section class="flex flex-col gap-2 lg:gap-4">
    <h1 class="text-2xl lg:text-4xl font-bold font-serif stricky">
        {tag.name}
    </h1>
    <h2>{tag.description}</h2>
    <hr />

    <hr />
    <div class="flex flex-col gap-4" datatype="vcard">
        {#each tag.resultList as post}
            <PostCard
                key={post.id}
                id={post.id}
                title={post.title}
                content={post.snippet}
                serie={post.serie}
                post_date={displayDate(post.publishDate)}
                link={post.url}
                tags={post.tags}
                class="mb-4"
            />
        {/each}
    </div>
    {#if tag.pageIndex + 1 < tag.totalPageNumber}
        <div class="flex items-center justify-center">
            <a
                class={cn([
                    buttonVariants.variants.variant.link,
                    buttonVariants.variants.size.lg,
                    "leading-12",
                ])}
                href="/posts/tags/{kebabCase(
                    tag.name,
                )}_{tag._sid}/{tag.pageIndex + 1}"
                hx-get="/posts/pr/tags/{kebabCase(
                    tag.name,
                )}_{tag._sid}/{tag.pageIndex + 1}"
                hx-trigger="click"
                hx-target="#parent-div"
                hx-swap="afterend transition:true scroll:bottom"
            >
                Load More
            </a>
        </div>
    {/if}
</section>
