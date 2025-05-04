import type { PageLoad, EntryGenerator } from "./$types";
import { resolveMetaTags } from "$lib/metas/index";
import type { MetaTagsProps, Twitter, OpenGraph } from "svelte-meta-tags";

import { BASE } from "$env/static/private";
import { error, redirect } from "@sveltejs/kit";
import { pathResolver } from "$lib/utils";

export const prerender = true;
export const csr = false;
export const load: PageLoad = async ({ fetch, params }) => {
  // console.log(params);
  if (!params.label || !params.obj_id || !params.pager) {
    error(404, {
      message: "Invalid Parameters",
    });
  }
  if (params.pager <= 0) {
    redirect(308, `/posts/series/${params.label}_${params.obj_id}`);
  }

  const index_req = await fetch("http://localhost:3000/series/index.json");

  if (!index_req.ok) {
    error(404, {
      message: "Series Index Not found",
    });
  }
  const series = await index_req.json();
  const seriesObj = series.find((s) => s._sid === params.obj_id);
  if (!seriesObj) {
    error(404, {
      message: "Series Not found",
    });
  }

  const serie = await fetch(
    `http://localhost:3000/series/${seriesObj.id}/p${params.pager}.json`,
  );

  if (!serie.ok) {
    error(404, {
      message: "Page Not found",
    });
  }

  return {
    serie: await serie.json(),
    meta: resolveTagsIndexPageMetaTags(seriesObj),
  };
};

function resolveTagsIndexPageMetaTags(tagObj: any) {
  const tag = tagObj.name;

  const title = `${tag} - Series Articles Index`;
  const description = `${tag}, all articles in series by Steven Dev's`;

  const twitter = {
    title,
    description,
  } satisfies Twitter;

  const openGraph = {
    title,
    description,
    url: `${BASE}/posts/tags/${tag}_${tagObj._sid}`,
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
          tag,
          "blog",
          "posts",
          "steven dev",
          "software developer",
          "developer",
          "engineering",
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

export const entries: EntryGenerator = async () => {
  const index_req = await fetch("http://localhost:3000/series/index.json");
  const index = await index_req.json();
  let allList = [];
  for (const serie of index) {
    if (serie.totalCount > 0) {
      const totalPageNum = Math.ceil(serie.totalCount / 150);
      allList.push(
        ...Array.from({ length: totalPageNum }, (_, i) => i).map((idx) => ({
          obj_id: serie._sid,
          label: pathResolver(serie.name),
          pager: `${idx}`,
        })),
      );
    } else {
      allList.push({
        obj_id: serie._sid,
        label: pathResolver(serie.name),
        pager: "0",
      });
    }
  }

  // console.log(allList);

  return allList;
};
