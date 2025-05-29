<script lang="ts">
    import { page } from "$app/stores";
    import PostCard from "$lib/components/post-layout/card";
    import { buttonVariants } from "$lib/components/ui/button";
    import { displayDate, cn } from "$lib/utils";
    import { MetaTags } from "svelte-meta-tags";
    import { kebabCase } from "lodash-es";
    import {
        Breadcrumb,
        BreadcrumbItem,
        BreadcrumbLink,
        BreadcrumbList,
        BreadcrumbPage,
        BreadcrumbSeparator,
    } from "$lib/components/ui/breadcrumb";
    import Slash from "svelte-radix/Slash.svelte";

    const { meta, serie } = $page.data;
</script>

<MetaTags {...meta} />

<section class="pt-2 pb-4 lg:pb-6">
    <Breadcrumb>
        <BreadcrumbList>
            <BreadcrumbItem>
                <BreadcrumbLink href="/">Home</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator>
                <Slash tabindex="-1" />
            </BreadcrumbSeparator>
            <BreadcrumbItem>
                <BreadcrumbLink href="/posts">Blog Post</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator>
                <Slash tabindex="-1" />
            </BreadcrumbSeparator>
            <BreadcrumbItem>
                <BreadcrumbLink href="/posts/series">Series</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator>
                <Slash tabindex="-1" />
            </BreadcrumbSeparator>
            <BreadcrumbItem>
                <BreadcrumbPage class="capitalize">{serie.name}</BreadcrumbPage>
            </BreadcrumbItem>
        </BreadcrumbList>
    </Breadcrumb>
</section>
<section class="flex flex-col gap-2 lg:gap-4">
    <h1 class="text-2xl lg:text-4xl font-bold font-serif stricky">
        {serie.name}
    </h1>
    <h2>{serie.description}</h2>
    <hr />

    <hr />
    <div id="parent-div" class="flex flex-col gap-4" datatype="vcard">
        {#each serie.resultList as post}
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
    {#if serie.pageIndex + 1 < serie.totalPageNumber}
        <div class="flex items-center justify-center">
            <a
                class={cn([
                    buttonVariants.variants.variant.link,
                    buttonVariants.variants.size.lg,
                    "leading-12",
                ])}
                href="/posts/series/{kebabCase(
                    serie.name,
                )}_{serie._sid}/{serie.pageIndex + 1}"
                hx-get="/posts/pr/series/{kebabCase(
                    serie.name,
                )}_{serie._sid}/{serie.pageIndex + 1}"
                hx-trigger="click"
                hx-target="#parent-div"
                hx-swap="afterend"
            >
                Load More
            </a>
        </div>
    {/if}
</section>
