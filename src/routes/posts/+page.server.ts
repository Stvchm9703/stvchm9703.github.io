import type { PageLoad, EntryGenerator } from "./$types";
import { resolveMetaTags } from "$lib/metas/index";
import type { MetaTagsProps, Twitter, OpenGraph } from "svelte-meta-tags";

import { BASE } from "$env/static/private";
import { error } from "@sveltejs/kit";

export const prerender = true;
// export const csr = true;
export const load: PageLoad = async ({ fetch, params }) => {
  const index_req = await fetch("http://localhost:3000/series/index.json");
  if (!index_req.ok) {
    error(404, {
      message: "Series Index Not found",
    });
  }

  const series = await index_req.json();
  const seriesName = series
    .filter((s) => s.resultList.length > 0)
    .map((s) => s.name);
  return {
    series,
    meta: resolvePostIndexPageMetaTags(seriesName),
  };
};

function resolvePostIndexPageMetaTags(series: string[]) {
  const title = "Blog Posts Index";
  const description = "A collection of blog posts by Steven Dev's";

  const twitter = {
    title,
    description,
  } satisfies Twitter;

  const openGraph = {
    title,
    description,
    url: `${BASE}/posts`,
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

// export const entries: EntryGenerator = async () => {
//   const index_req = await fetch("http://localhost:3000/index.json");
//   const index = await index_req.json();
//   return index;
// };
