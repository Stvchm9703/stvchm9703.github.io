import type { PageLoad, EntryGenerator } from "./$types";

export let prerender = true;
export const csr = false;
export const load: PageLoad = async ({ fetch, params }) => {
  const index_req = await fetch("http://localhost:3000/series.json");
  const series = await index_req.json();

  return { series };
};

// export const entries: EntryGenerator = async () => {
//   const index_req = await fetch("http://localhost:3000/index.json");
//   const index = await index_req.json();
//   return index;
// };
