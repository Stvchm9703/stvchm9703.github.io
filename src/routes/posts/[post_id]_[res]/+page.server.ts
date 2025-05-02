import type { PageLoad, EntryGenerator } from "./$types";
export const csr = false;
export let prerender = true;

const blogPostIndex = () => import("/blog_post_resolved/index.json");

export const load: PageLoad = async ({ fetch, params }) => {
  const index_req = await fetch("http://localhost:3000/index.json");
  const index = await index_req.json();

  const target_post = index.find((item) => item.post_id === params.post_id);

  if (!target_post) {
    throw new Error(`Post with ID ${params.post_id} not found`);
  }

  const content = await fetch(
    `http://localhost:3000/${target_post.page_content_path.replace("blog_post_resolved/", "")}`,
  );

  return await content.json();
};

export const entries: EntryGenerator = async () => {
  const index_req = await fetch("http://localhost:3000/index.json");
  const index = await index_req.json();
  return index;
};
