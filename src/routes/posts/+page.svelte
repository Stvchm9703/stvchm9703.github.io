<script>
    import { Separator } from "$lib/components/ui/separator";
    import Headline from "$lib/post-layout/headline.svelte";
    import PostContainer from "$lib/post-layout/post-list-container.svelte";
    import PostCard from "$lib/post-layout/card";
    let { post_list } = $props();
</script>

<Headline head_title="first post" content="sample content" />

<Separator class="w-full" />

<section class="w-full flex justify-between">
    <PostContainer>
        {#each Array.from({ length: 6 }) as _, idx}
            <PostCard
                key={idx}
                title="Title left {idx}"
                content="Content {idx}"
                post_date="2023-01-01"
                link="https://example.com"
            />
        {/each}
    </PostContainer>
    <Separator orientation="vertical" />
    <PostContainer>
        {#each Array.from({ length: 8 }) as _, idx}
            <PostCard
                key={idx}
                title="Title center {idx}"
                content="Content {idx}"
                post_date="2023-01-01"
                link="https://example.com"
            />
        {/each}
    </PostContainer>
    <Separator orientation="vertical" />
    <PostContainer>
        {#each Array.from({ length: 7 }) as _, idx}
            <PostCard
                key={idx}
                title="Title right {idx}"
                content="Content {idx}"
                post_date="2023-01-01"
                link="https://example.com"
            />
        {/each}
    </PostContainer>
</section>
