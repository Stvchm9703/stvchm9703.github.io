import type { PageLoad } from "./$types";

export let prerender = true;
export const load: PageLoad = async ({ fetch }) => {
  // const res = await fetch("/my-server-route.json");
  // return await res.json();
  return {
    title: "My Blog Post",
    content: "This is my blog post content.",
  };
};

export const entries: EntryGenerator = () => {
  return [
    /// test route for prerender
    { post_id: "1", res: "hello-world" },
    { post_id: "2", res: "another-blog-post" },
  ];
};
