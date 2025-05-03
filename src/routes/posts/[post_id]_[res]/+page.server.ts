import type { PageLoad, EntryGenerator } from "./$types";
import type { Page as IPage } from "$generateor/page";
import { resolveMetaTags } from "$lib/metas/index";
import type {
  MetaTag,
  MetaTagsProps,
  OpenGraph,
  Twitter,
} from "svelte-meta-tags";
import { displayDate } from "$lib/utils";
import { BASE } from "$env/static/private";
// export const csr = false;
export let prerender = true;

export const load: PageLoad = async ({ fetch, params }) => {
  const index_req = await fetch("http://localhost:3000/index.json");
  const index = await index_req.json();

  const target_post = index.find((item) => item.post_id === params.post_id);

  if (!target_post) {
    throw new Error(`Post with ID ${params.post_id} not found`);
  }

  const content_result = await fetch(
    `http://localhost:3000/${target_post.page_content_path.replace("blog_post_resolved/", "")}`,
  );

  const content = (await content_result.json()) as IPage;
  const meta = resolvePostDetailMetaTags(content);

  return {
    content,
    meta,
  };
};

export const entries: EntryGenerator = async () => {
  const index_req = await fetch("http://localhost:3000/index.json");
  const index = await index_req.json();
  return index;
};

function resolvePostDetailMetaTags(post: IPage) {
  const title = post.title;
  const description = post.snippet;
  const tags = [
    post.serie?.name ?? "",
    ...post.tags.map((tag) => tag.name),
  ].filter((p) => p !== "");
  const twitter = {
    cardType: "summary_large_image",
    image: post.coverImage ?? undefined,
    imageAlt: `cover image, ${post.title}, Steven Dev's`,
  } satisfies Twitter;
  const openGraph = {
    type: "article",
    title,
    description,
    article: {
      publishedTime: new Date((post.publish_date ?? 0) * 1000).toISOString(),
      // modifiedTime: post.updatedAt,
      authors: [BASE],
      tags,
    },
    images: [
      post.coverImage
        ? {
            url: `${BASE}/blog/assets/files/${post.coverImage}`,
            width: 850,
            height: 650,
            alt: "cover image",
          }
        : undefined,
      ...(post.meta?.images ?? []),
    ]
      .filter((p) => p)
      .map((e) => ({ ...e, url: `${BASE}${e.url}` })),
    videos: post.meta?.videos ?? undefined,
    audio: post.meta?.audio ?? undefined,
  } satisfies OpenGraph;

  const base = {
    title,
    description,
    twitter,
    openGraph,
    additionalMetaTags: [
      {
        name: "keywords",
        content: [...tags, "Developer", "Software Engineer"],
      },
    ],
  } as MetaTagsProps;

  return resolveMetaTags(base);
}
