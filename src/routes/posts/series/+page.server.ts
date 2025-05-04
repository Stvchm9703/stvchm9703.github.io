import type { PageLoad, EntryGenerator } from "./$types";
import { resolveMetaTags } from "$lib/metas/index";
import type { MetaTagsProps, Twitter, OpenGraph } from "svelte-meta-tags";

import { BASE } from "$env/static/private";

export const prerender = true;
export const csr = false;
export const load: PageLoad = async ({ fetch, params }) => {
  const index_req = await fetch("http://localhost:3000/series/index.json");
  const series = await index_req.json();
  const tagsName = series
    .filter((s) => s.resultList.length > 0)
    .map((s) => s.name);
  return {
    series,
    meta: resolveTagsIndexPageMetaTags(tagsName),
  };
};

function resolveTagsIndexPageMetaTags(series: string[]) {
  const title = "Blog Posts Series Index";
  const description = "Series Articles posted by Steven Dev's";

  const twitter = {
    title,
    description,
  } satisfies Twitter;

  const openGraph = {
    title,
    description,
    url: `${BASE}/posts/series`,
  } satisfies OpenGraph;

  const tmpBase: MetaTagsProps = {
    title,
    titleTemplate: `%s | Steven Dev's Blog`,

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
