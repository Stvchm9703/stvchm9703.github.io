<script>
    import CalendarIcon from "lucide-svelte/icons/calendar";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import Image from "$lib/components/Image.svelte";
    import Tag from "$lib/components/post-content-layout/tag/tag.svelte";
    import { cn, displayDate } from "$lib/utils";
    const { post, content_slot } = $props();
import { buttonVariants } from "$lib/components/ui/button";
</script>

<section class="py-4">
    <!-- {/* Main Headline */} -->
    <div class="mb-10">
        <h2 class="font-serif text-2xl md:text-5xl font-bold mb-4 capitalize">
            {post.label}
        </h2>
        <div class="flex gap-2 text-muted-foreground mb-4 items-center">
            {#if post.serie}
                    <a
                        class="block text-sm text-slate-800 hover:text-slate-500 hover:underline"
                        href={post.serie.url}
                        target="_self"
                    >
                        {post.serie.label}
                    </a>

                <i class="h-4 border-r-1 border-r-slate-400 border-r-solid -mb-1"></i>
            {/if}
            <div class="flex items-center gap-1">
                <CalendarIcon class="h-4 w-4" />
                <span class="text-sm">{displayDate(post.publishDate)}</span>
            </div>
            <i class="h-4 border-r-1 border-r-slate-400 border-r-solid -mb-1"></i>
            <span class="text-sm">{"steven"}</span>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div class="md:col-span-2 flex flex-col justify-between">
                <div class="flex flex-col gap-4">
                    {@render content_slot?.()}
                    {#if !content_slot}
                    <p class="text-lg font-serif leading-relaxed">
                        {post.snippet}
                    </p>
                    <div class="flex gap-4 mt-3">
                        {#each post.tags as tag} <Tag {...tag} /> {/each}
                    </div>
                    {/if}
                </div>
                <div class="flex flex-col gap-4 items-end">
                    <a class={cn([buttonVariants({variant:'link', size:'sm'})])}
                        href={post.url}
                        target="_self"
                        rel="noopener noreferrer">
                        Read More
                    </a>
                </div>
            </div>
            <Image
                class="lg:min-h-72 h-64 md:h-full"
                baseImagePath={null}
                imagePath={null}
                alt="Technology illustration"
            />
        </div>
    </div>
    <Separator />
</section>
