<script>
    import { cn, pathResolver, once, preventDefault } from "$lib/utils";
    import { kebabCase } from "lodash-es";

    import { page } from "$app/state";
    import Tag from "$lib/components/post-content-layout/tag/tag.svelte";
    import { MetaTags } from "svelte-meta-tags";
    import {
        Breadcrumb,
        BreadcrumbItem,
        BreadcrumbLink,
        BreadcrumbList,
        BreadcrumbPage,
        BreadcrumbSeparator,
    } from "$lib/components/ui/breadcrumb";

    import Slash from "svelte-radix/Slash.svelte";

    const { tags, meta } = page.data;
    const handleTagClick = (tag) => {
        console.log(`Tag ${tag.name} clicked`);
    };

    // const tagList = tags.filter((item) => item.totalCount > 0) || [];
    const tagList = tags;
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
                <BreadcrumbPage>Tags</BreadcrumbPage>
            </BreadcrumbItem>
        </BreadcrumbList>
    </Breadcrumb>
</section>
<section>
    <h1 class="text-2xl lg:text-4xl font-bold font-serif stricky py-2 lg:pb-4">
        Tags
    </h1>

    <div aria-label="word-cloud" class="w-full"></div>

    <div class="flex gap-4">
        {#each tagList as tag}
            <Tag
                {...tag}
                label={`${tag.name} (${tag.totalCount})`}
                url="/posts/tags/{kebabCase(tag.name)}_{tag._sid}"
                onclick={preventDefault(() => handleTagClick(tag))}
            />
        {/each}
    </div>
</section>
