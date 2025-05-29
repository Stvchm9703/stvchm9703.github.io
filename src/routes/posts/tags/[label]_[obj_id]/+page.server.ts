import type { PageLoad, EntryGenerator } from "./$types";
import { resolveMetaTags } from "$lib/metas/index";
import type { MetaTagsProps, Twitter, OpenGraph } from "svelte-meta-tags";

import { BASE } from "$env/static/private";
import { pathResolver } from "$generateor/common";
import { kebabCase } from "lodash-es";
import { error } from "@sveltejs/kit";

export const prerender = true;
export const csr = false;
export const load: PageLoad = async ({ fetch, params }) => {
  console.log(params);
  const index_req = await fetch("http://localhost:3000/tags/index.json");
  const tags = await index_req.json();
  const tagObj = tags.find((s) => s._sid === params.obj_id);

  if (!tagObj) {
    error(404, {
      message: "Series Not found",
    });
  }

  const tag = await fetch(
    `http://localhost:3000/tags/${kebabCase(tagObj.name)}_${tagObj._sid}/p0.json`,
  );

  return {
    tag: await tag.json(),
    meta: resolveTagsIndexPageMetaTags(tagObj),
  };
};

function resolveTagsIndexPageMetaTags(tagObj: any) {
  const tag = tagObj.name;

  const title = `${tag} - Posts Index`;
  const description = `${tag}, posts around ${tag} by Steven Dev's`;

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
  const index_req = await fetch("http://localhost:3000/tags/index.json");
  const index = await index_req.json();
  return index.map((elm) => ({
    obj_id: elm._sid,
    label: pathResolver(elm.name),
  }));
};
