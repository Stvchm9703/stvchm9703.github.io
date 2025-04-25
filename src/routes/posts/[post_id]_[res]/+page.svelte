<script lang="ts">
    import { page } from "$app/state";
    import { Button } from "$lib/components/ui/button";
    import { Blocks, Calendar } from "lucide-svelte";
    import * as Breadcrumb from "$lib/components/ui/breadcrumb";
    import Slash from "svelte-radix/Slash.svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import TableOfContents from "$lib/post-content-layout/table-of-content/container.svelte";
    import TableOfContentsMobile from "$lib/post-content-layout/table-of-content/mob-list.svelte";
    import RelatedChapters from "$lib/post-content-layout/related-chapter/container.svelte";
    import RelatedChaptersMobile from "$lib/post-content-layout/related-chapter/mob-list.svelte";
    import Tag from "$lib/post-content-layout/tag/tag.svelte";
    import PostCard from "$lib/post-layout/card/base-card.svelte";

    // CMS - content block render Component
    // import BlockHeading from "$lib/post-content-layout/block/heading.svelte";
    // import BlockParagraph from "$lib/post-content-layout/block/paragraph.svelte";
    import BlockCode from "$lib/post-content-layout/block/code.svelte";
    // import BlockImage from "$lib/post-content-layout/block/image.svelte";
    import BlockLatex from "$lib/post-content-layout/block/latex.svelte";
    // import BlockQuote from "$lib/post-content-layout/block/quote.svelte";
    // import BlockVideo from "$lib/post-content-layout/block/video.svelte";
    import BlockText from "$lib/post-content-layout/block/text.svelte";
    import BlockTable from "$lib/post-content-layout/block/table.svelte";

    import BlockBookmark from "$lib/post-content-layout/block/bookmark.svelte";
    import BlockLink from "$lib/post-content-layout/block/link.svelte";

    import BlockFile from "$lib/post-content-layout/block/file.svelte";

    import type { Page as IPage } from "$generateor/page";

    const post = page.data as IPage;

    // console.log(post);

    // import example_post from "./example";
    // const post: PostContent = example_post;
    // import type { PostContent } from "$types/post-content";
    // console.log(page.data);

    function resolveComponent(type: string) {
        switch (type) {
            // case "image":
            //     return BlockImage;
            // case "video":
            //     return BlockVideo;
            case "Text":
                return BlockText;
            case "Link":
                return BlockLink;
            // case "datatable":
            case "Table":
                return BlockTable;
            //     // !todo
            //     return null;
            default:
                return null;
        }
    }
</script>

<svelte:head>
    <title>{post.title} - Steven Dev;s Log</title>
    <meta name="description" content={post.snippet} />
</svelte:head>

<Breadcrumb.Root class="py-4">
    <Breadcrumb.List>
        <Breadcrumb.Item>
            <Breadcrumb.Link href="/">Home</Breadcrumb.Link>
        </Breadcrumb.Item>
        <Breadcrumb.Separator>
            <Slash tabindex="-1" />
        </Breadcrumb.Separator>
        <Breadcrumb.Item>
            <Breadcrumb.Link href="/posts">Blog Post</Breadcrumb.Link>
        </Breadcrumb.Item>
        <Breadcrumb.Separator>
            <Slash tabindex="-1" />
        </Breadcrumb.Separator>

        {#if post.serie !== null}
            <Breadcrumb.Item>
                <Breadcrumb.Link href="/posts/serie/{post.serie.id}"
                    >{post.serie.name}</Breadcrumb.Link
                >
            </Breadcrumb.Item>
            <Breadcrumb.Separator>
                <Slash tabindex="-1" />
            </Breadcrumb.Separator>
        {/if}

        <Breadcrumb.Item>
            <Breadcrumb.Page>{post.title}</Breadcrumb.Page>
        </Breadcrumb.Item>
    </Breadcrumb.List>
</Breadcrumb.Root>

<section class="flex flex-col lg:flex-row gap-8">
    <!-- {/* Sidebar with TOC and Related Chapters */} -->
    <aside class="lg:w-64 xl:w-72 flex-none">
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
                        <Tag {...tag} />
                    {/each}
                </div>
                <h2
                    class="font-serif text-4xl md:text-5xl font-bold mb-4"
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
                            >{new Date(
                                post.publish_date * 1000,
                            ).toLocaleDateString()}</span
                        >
                    </div>
                    <!-- <div class="flex items-center gap-2">
                        <i class="w-4 h-4 i-carbon-time"></i>
                        <span class="text-sm">{post.readTime}</span>
                    </div> -->
                    <div class="flex items-center gap-2">
                        <i class="w-4 h-4 i-carbon-user"></i>
                        <span class="text-sm"> By Steven, </span>
                    </div>
                </div>
            </div>

            <!-- {/* Article Content */} -->
            <div class="font-serif leading-relaxed mb-12">
                {#each post.reformedContents as block, index}
                    <!-- {@html block} -->
                    <!-- <svelte:component
                        this={resolveComponent(block.type)}
                        key={index}
                        {block}
                    /> -->
                    {#if block.componentType === "Text" && block.componentAttr["style"] === "Code"}
                        <BlockCode {...block} />
                    {:else if block.componentType === "Text"}
                        <BlockText {...block} />
                    {/if}
                    {#if block.componentType === "Table"}
                        <BlockTable {...block} />
                    {/if}

                    {#if block.componentType === "Bookmark"}
                        <BlockBookmark {...block} />
                    {/if}
                    <!--
                    {#if block.componentType === "Image"}
                        <BlockImage {...block} />
                    {/if} -->
                    {#if block.componentType === "File"}
                        <BlockFile {...block} />
                    {/if}

                    {#if block.componentType === "Link"}
                        <BlockLink {...block} />
                    {/if}
                    {#if block.componentType === "Latex"}
                        <BlockLatex {...block} />
                    {/if}

                    <!-- <span class="block">
                        {block.componentType} - {JSON.stringify(
                            block.componentAttr,
                        )}
                    </span> -->
                {/each}
            </div>
        </article>
        <!-- {/* Related Posts */} -->
        {#if post.relatedPosts.length > 0}
            <div class="mb-12">
                <h4 class="font-serif text-2xl font-bold mb-4">
                    Related Articles
                </h4>
                <hr class="mb-6" />
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <!-- {post.relatedPosts.map((relatedPost) => ( -->
                    {#each post.relatedPosts as relatedPost}
                        <a
                            id={relatedPost.id}
                            key={relatedPost.id}
                            href={relatedPost.url}
                            class="group"
                        >
                            <article
                                class="border p-4 h-full hover:bg-muted/50 transition-colors"
                            >
                                <h5
                                    class="font-serif text-lg font-bold group-hover:underline"
                                >
                                    {relatedPost.title}
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
