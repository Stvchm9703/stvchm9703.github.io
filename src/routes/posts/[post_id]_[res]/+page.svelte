<script lang="ts">
    import { page } from "$app/state";
    import { Button } from "$lib/components/ui/button";
    import { Blocks, Calendar } from "lucide-svelte";

    import Separator from "$lib/components/ui/separator/separator.svelte";
    import TableOfContents from "$lib/post-content-layout/table-of-content/container.svelte";
    import TableOfContentsMobile from "$lib/post-content-layout/table-of-content/mob-list.svelte";
    import RelatedChapters from "$lib/post-content-layout/related-chapter/container.svelte";
    import RelatedChaptersMobile from "$lib/post-content-layout/related-chapter/mob-list.svelte";
    import PostCard from "$lib/post-layout/card/base-card.svelte";

    // CMS - content block render Component
    import BlockHeading from "$lib/post-content-layout/block/heading.svelte";
    import BlockParagraph from "$lib/post-content-layout/block/paragraph.svelte";
    import BlockCode from "$lib/post-content-layout/block/code.svelte";
    import BlockImage from "$lib/post-content-layout/block/image.svelte";
    import BlockQuote from "$lib/post-content-layout/block/quote.svelte";
    import BlockVideo from "$lib/post-content-layout/block/video.svelte";

    const { title, content } = page.data;

    import example_post from "./example";
    const post: PostContent = example_post;
    import type { PostContent } from "$types/post-content";

    function resolveComponent(type: string) {
        switch (type) {
            case "heading":
                return BlockHeading;
            case "paragraph":
                return BlockParagraph;
            case "code":
                return BlockCode;
            case "image":
                return BlockImage;
            case "quote":
                return BlockQuote;
            case "video":
                return BlockVideo;
            // case "datatable":
            // case "table":
            //     // !todo
            //     return null;
            default:
                return null;
        }
    }
</script>

<section class="flex flex-col lg:flex-row gap-8">
    <!-- {/* Sidebar with TOC and Related Chapters */} -->
    <aside class="lg:w-64 xl:w-72 shrink-0">
        <!-- {/* Table of Contents - Desktop */} -->
        <div class="hidden lg:block sticky top-8">
            <TableOfContents tableOfContents={post.tableOfContents} />
            <!-- {/* Related Chapters */} -->
            <RelatedChapters relatedChapters={post.relatedChapters} />
        </div>

        <!-- {/* Table of Contents - Mobile Dropdown */} -->
        <TableOfContentsMobile tableOfContents={post.tableOfContents} />

        <!-- {/* Related Chapters - Mobile */} -->
        <RelatedChaptersMobile relatedChapters={post.relatedChapters} />
    </aside>

    <!-- {/* Main Content */} -->
    <div class="flex-1">
        <!-- {/* Article Header */} -->
        <article>
            <div class="mb-8">
                <div class="flex flex-wrap gap-2 mb-3">
                    <!-- {post.tags.map((tag, index) => ( -->
                    {#each post.tags as tag, index}
                        <a
                            href={`/tag/${tag.toLowerCase().replace(/\s+/g, "-")}`}
                            key={index}
                            class="bg-muted text-xs px-2 py-1 rounded hover:bg-muted/80"
                        >
                            {tag}
                        </a>
                    {/each}
                    <!-- ))} -->
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
                        <span class="text-sm">{post.date}</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <i class="w-4 h-4 i-carbon-time"></i>
                        <span class="text-sm">{post.readTime}</span>
                    </div>
                    <div class="flex items-center gap-2">
                        <i class="w-4 h-4 i-carbon-user"></i>
                        <span class="text-sm">
                            By {post.author}, {post.authorRole}
                        </span>
                    </div>
                </div>
            </div>

            <!-- {/* Article Content */} -->
            <div class="font-serif text-lg leading-relaxed space-y-6 mb-12">
                {#each post.content as block, index}
                    <svelte:component
                        this={resolveComponent(block.type)}
                        key={index}
                        {block}
                    />
                {/each}
            </div>
        </article>
        <!-- {/* Related Posts */} -->
        <div class="mb-12">
            <h4 class="font-serif text-2xl font-bold mb-4">Related Articles</h4>
            <hr class="mb-6" />
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <!-- {post.relatedPosts.map((relatedPost) => ( -->
                {#each post.relatedPosts as relatedPost}
                    <a
                        href={`/post/${relatedPost.id}`}
                        key={relatedPost.id}
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
