<script lang="ts">
    import "uno.css";

    import { page } from "$app/state";
    import { Button } from "$lib/components/ui/button";
    // import { Blocks, Calendar } from "lucide-svelte";
    import {
        initLightGallery,
        // lightgallery,
    } from "$lib/stores/lightgallery.svelte";
    import {
        Breadcrumb,
        BreadcrumbItem,
        BreadcrumbLink,
        BreadcrumbList,
        BreadcrumbPage,
        BreadcrumbSeparator,
    } from "$lib/components/ui/breadcrumb";

    import Slash from "svelte-radix/Slash.svelte";
    // import Separator from "$lib/components/ui/separator/separator.svelte";
    import TableOfContents from "$lib/components/post-content-layout/table-of-content/container.svelte";
    import TableOfContentsMobile from "$lib/components/post-content-layout/table-of-content/mob-list.svelte";
    import RelatedChapters from "$lib/components/post-content-layout/related-chapter/container.svelte";
    import RelatedChaptersMobile from "$lib/components/post-content-layout/related-chapter/mob-list.svelte";
    import Tag from "$lib/components/post-content-layout/tag/tag.svelte";
    // import PostCard from "$lib/components/post-layout/card/base-card.svelte";

    // CMS - content block render Component
    import BlockLayout from "$lib/components/post-content-layout/block/layout.svelte";

    import { isEmpty } from "lodash-es";

    import type { Page as IPage } from "$generateor/page";
    import { displayDate } from "$lib/utils";
    import { MetaTags } from "svelte-meta-tags";
    import { onMount } from "svelte";
    // import { mel } from "svelte-highlight/languages";
    const { content: post, meta } = page.data;
    // const {  } = page.data;

    /** @type {HTMLDivElement} */
    let articleContent;
    onMount(() => {
        // console.log(post);
        initLightGallery(articleContent, post.meta.images);
    });
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
            {#if isEmpty(post.serie) == false}
                <BreadcrumbItem>
                    <BreadcrumbLink href={post.serie?.url}>
                        {post.serie?.label}
                    </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator>
                    <Slash tabindex="-1" />
                </BreadcrumbSeparator>
            {/if}

            <BreadcrumbItem>
                <BreadcrumbPage>{post.title}</BreadcrumbPage>
            </BreadcrumbItem>
        </BreadcrumbList>
    </Breadcrumb>
</section>

<section class="flex flex-col lg:flex-row gap-4 lg:gap-8">
    <!-- {/* Sidebar with TOC and Related Chapters */} -->
    <aside class="w-full lg:w-64 xl:w-72 flex-none">
        <!-- {/* Table of Contents - Desktop */} -->
        <div class="hidden lg:block sticky top-8">
            <TableOfContents tableOfContents={post.tableOfContents} />
            <!-- {/* Related Chapters */} -->
            {#if post.relatedChapters.length > 0}
                <RelatedChapters relatedChapters={post.relatedChapters} />
            {/if}
        </div>

        <!-- {/* Table of Contents - Mobile Dropdown */} -->
        <TableOfContentsMobile tableOfContents={post.tableOfContents} />

        <!-- {/* Related Chapters - Mobile */} -->
        {#if post.relatedChapters.length > 0}
            <RelatedChaptersMobile relatedChapters={post.relatedChapters} />
        {/if}
    </aside>

    <!-- {/* Main Content */} -->
    <div class="flex-none w-auto grow-1 shrink-1 basis-[56rem] max-w-[58rem]">
        <!-- {/* Article Header */} -->
        <article>
            <div class="mb-8">
                <div class="flex flex-wrap gap-2 mb-3">
                    {#each post.tags as tag}
                        <Tag {...tag} key={tag.id} />
                    {/each}
                </div>
                <h2
                    class="font-serif text-3xl md:text-5xl font-bold mb-4"
                    id="introduction"
                >
                    {post.title}
                </h2>
                <div
                    class="flex flex-wrap items-center gap-4 text-muted-foreground mb-6"
                >
                    <div class="flex items-center gap-2">
                        <!-- <Calendar class="w-4 h-4 text-current" /> -->
                        <i class="w-4 h-4 i-carbon-calendar"></i>
                        <span class="text-sm"
                            >{displayDate(post.publishDate)}</span
                        >
                    </div>
                    <div class="flex items-center gap-2">
                        <i class="w-4 h-4 i-carbon-user"></i>
                        <span class="text-sm"> By Steven, </span>
                    </div>
                </div>
            </div>

            <!-- {/* Article Content */} -->
            <div
                bind:this={articleContent}
                class="font-serif leading-relaxed mb-12"
            >
                {#each post.contents as block}
                    <BlockLayout {...block} />
                {/each}
            </div>
        </article>
        <!-- {/* Related Posts */} -->
        {#if post.relatedArticles.length > 0}
            <div class="mb-12">
                <h4 class="font-serif text-2xl font-bold mb-4">
                    Related Articles
                </h4>
                <hr class="mb-6" />
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <!-- {post.relatedPosts.map((relatedPost) => ( -->
                    {#each post.relatedArticles.slice(0, 3) as relatedPost}
                        <a
                            id={relatedPost.id}
                            key={relatedPost.id}
                            href={relatedPost.url}
                            class="group"
                            target="_self"
                        >
                            <article
                                title={relatedPost.label}
                                class="border p-4 h-full hover:bg-muted/50 transition-colors"
                            >
                                <h5
                                    class="font-serif text-lg font-bold group-hover:underline"
                                >
                                    {relatedPost.label}
                                </h5>
                            </article>
                        </a>
                    {/each}
                    <!-- ))} -->
                </div>
            </div>
        {/if}
        <!-- {/* Pagination */} -->
        <div
            class="flex justify-between items-center border-t border-b py-6 my-8"
        >
            <Button
                class="flex items-center gap-2 hover:bg-muted rounded-md transition-colors"
            >
                <i class="w-4 h-4 i-carbon-chevron-left"></i>
                <span>Previous Article</span>
            </Button>
            <Button
                class="flex items-center gap-2  hover:bg-muted rounded-md transition-colors"
            >
                <span>Next Article</span>
                <i class="w-4 h-4 i-carbon-chevron-right"></i>
            </Button>
        </div>
    </div>
</section>

<style>
    /* @import "lightgallery/css/lightgallery.css"; */
    /* @import "lightgallery/css/lg-thumbnail.css"; */
    /* @import "lightgallery/css/lg-zoom.css"; */
</style>
