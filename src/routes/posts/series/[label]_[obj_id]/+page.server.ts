import type { PageLoad, EntryGenerator } from "./$types";

import { error } from "@sveltejs/kit";

import { resolveMetaTags } from "$lib/metas/index";
import type { MetaTagsProps, Twitter, OpenGraph } from "svelte-meta-tags";

import { BASE } from "$env/static/private";
import { pathResolver } from "$lib/utils";
import { kebabCase } from "lodash-es";

export const prerender = true;
export const csr = false;
export const load: PageLoad = async ({ fetch, params }) => {
  // console.log(params);
  const index_req = await fetch("http://localhost:3000/series/index.json");
  const series = await index_req.json();
  const seriesObj = series.find((s) => s._sid === params.obj_id);
  if (!seriesObj) {
    error(404, {
      message: "Series Not found",
    });
  }

  const serie = await fetch(
    `http://localhost:3000/series/${kebabCase(seriesObj.name)}_${seriesObj._sid}/p0.json`,
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
    url: `${BASE}/posts/series/${tag}_${tagObj._sid}`,
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

export const entries: EntryGenerator = async () => {
  const index_req = await fetch("http://localhost:3000/series/index.json");
  const index = await index_req.json();
  return index.map((elm) => ({
    obj_id: elm._sid,
    label: pathResolver(elm.name),
  }));
};
