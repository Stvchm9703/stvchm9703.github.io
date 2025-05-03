import { resolveMetaTags } from "$lib/metas";

export const prerender = true;

export const load = async ({ fetch }) => {
  return {
    meta: resolveMetaTags({
      title: "Steven, the web dev",
      titleTemplate: "%s",
    }),
  };
};
