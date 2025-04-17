<script>
    import { CalendarIcon } from "lucide-svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import Image from "$lib/Image.svelte";
    let { head_title, post_date, content, content_slot } = $props();

</script>

<section class="py-4">
    <!-- {/* Main Headline */} -->
    <div class="mb-10">
        <h2 class="font-serif text-4xl md:text-5xl font-bold mb-4 capitalize">
            {head_title}
        </h2>
        <div class="flex gap-2 text-muted-foreground mb-4">
            <CalendarIcon class="h-4 w-4" />
            <span class="text-sm">{post_date}</span>
            <span class="mx-2">•</span>
            <span class="text-sm">{"steve"}</span>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div class="md:col-span-2">
                {@render content_slot?.()}
                {#if !content_slot}
                <p class="text-lg font-serif leading-relaxed">
                    {content}
                </p>
                {/if}
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
