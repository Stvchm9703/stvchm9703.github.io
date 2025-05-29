import type { EntryGenerator } from "./$types";
import { error } from "@sveltejs/kit";
import { displayDate, pathResolver } from "$lib/utils";
import { kebabCase } from "lodash-es";
export const prerender = true;
// export const csr = false;

export const entries: EntryGenerator = async () => {
  const index_req = await fetch("http://localhost:3000/series/index.json");
  const index = await index_req.json();
  let allList = [];
  for (const serie of index) {
    if (serie.totalCount > 0) {
      // const totalPageNum = Math.ceil(serie.totalCount / 150);
      allList.push(
        ...Array.from({ length: serie.totalPageNumber }, (_, i) => i).map(
          (idx) => ({
            id: serie._sid,
            label: kebabCase(serie.name),
            pager: `${idx}`,
          }),
        ),
      );
    } else {
      allList.push({
        id: serie._sid,
        label: kebabCase(serie.name),
        pager: "0",
      });
    }
  }

  // console.log(allList);

  return allList;
};

import type { RequestHandler } from "./$types";
import PostCard from "$lib/components/post-layout/card";
import { render } from "svelte/server";
export const GET: RequestHandler = async ({ url, params }) => {
  const { label, id, pager } = params;
  if (!label || !id || !pager) {
    error(404, {
      message: "Invalid Parameters",
    });
  }
  console.log(label, id, pager);
  const index_req = await fetch(
    `http://localhost:3000/series/${kebabCase(label)}_${id}/p${pager}.json`,
  );
  // console.log(index_req);

  if (!index_req.ok) {
    error(404, {
      message: "Series Page Not found",
    });
  }

  const page = await index_req.json();

  // console.log(page);
  const renderPage = page.resultList
    .map((post) => {
      return render(PostCard, {
        props: {
          id: post.id,
          title: post.label,
          content: post.snippet,
          serie: post.serie,
          post_date: displayDate(post.publishDate),
          link: post.url,
          tags: post.tags,
        },
      }).html;
    })
    .join("");

  return new Response(String(renderPage), {
    headers: {
      "Content-Type": "text/html",
    },
  });
};
