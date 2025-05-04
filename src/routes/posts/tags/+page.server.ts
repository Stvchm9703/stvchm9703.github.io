import type { PageLoad, EntryGenerator } from "./$types";
import { resolveMetaTags } from "$lib/metas/index";
import type { MetaTagsProps, Twitter, OpenGraph } from "svelte-meta-tags";

import { BASE } from "$env/static/private";
import { pathResolver } from "$lib/utils";

export const prerender = true;
export const csr = false;
export const load: PageLoad = async ({ fetch, params }) => {
  const index_req = await fetch("http://localhost:3000/tags/index.json");
  const tags = await index_req.json();
  const tagsName = tags
    .filter((s) => s.resultList.length > 0)
    .map((s) => s.name);
  return {
    tags,
    meta: resolveTagsIndexPageMetaTags(tagsName),
  };
};

function resolveTagsIndexPageMetaTags(series: string[]) {
  const title = "Blog Posts Tag Index";
  const description = "Topic of posts by Steven Dev's";

  const twitter = {
    title,
    description,
  } satisfies Twitter;

  const openGraph = {
    title,
    description,
    url: `${BASE}/posts/tags`,
  } satisfies OpenGraph;

  const tmpBase: MetaTagsProps = {
    title,
    description,
    twitter,
    openGraph,
    additionalMetaTags: [
      {
        name: "keywords",
        content: [
          "blog",
          "posts",
          "steven dev",
          "software developer",
          "developer",
          "engineering",
          ...series,
        ],
      },
    ],
  };

  return resolveMetaTags(tmpBase);
}

export const entries: EntryGenerator = async () => {
  const index_req = await fetch("http://localhost:3000/tags/index.json");
  const index = await index_req.json();
  return index.map((elm) => ({
    obj_id: elm._sid,
    label: pathResolver(elm.name),
  }));
};
