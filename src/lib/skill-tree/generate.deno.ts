// import Fuse from 'https://deno.land/x/fuse/dist/fuse.esm.js'

// import { nodes } from './data.js'

// import repoList from '../../assets/content/gh-repo-all.json'

import { Octokit } from "https://esm.sh/@octokit/rest";
import { createTokenAuth } from "https://esm.sh/@octokit/auth-token";
import { delay } from "https://deno.land/x/delay@v0.2.0/mod.ts";

const nodes = [
  // layer 2 limit-6
  {
    id: 6,
    name: "Vue",
    query: ["language:vue"],
    layer: 2,
    parent: 1,
    color: "#41b883",
  },
  { id: 7, name: "Nuxt", layer: 3, parent: 6, color: "#00c58e" },
  {
    id: 8,
    name: "Svelte",
    query: ["language:svelte"],
    layer: 2,
    parent: 1,
    color: "#ff3e00",
  },
  {
    id: 9,
    name: "Astro",
    query: ["path:*.astro"],
    layer: 2,
    parent: 1,
    color: "#000000",
  },
  // { id: 10, name: 'others', layer: 2, parent: 1, color: '#607d8b' },

  // layer 3 limit-9
  {
    id: 11,
    name: "Node",
    query: ["lanaguage:js", "path:*.js", "path:*.jsx"],
    layer: 2,
    parent: 1,
    color: "#68a063",
  },
  {
    id: 12,
    name: "solid-js",
    query: ["lanaguage:jsx", "path:*.jsx"],
    layer: 2,
    parent: 1,
    color: "#4377bb",
  },
  { id: 13, name: "Express", layer: 3, parent: 11, color: "#000000" },
  { id: 14, name: "Deno", layer: 3, parent: 10, color: "#000000" },
  { id: 15, name: "HTMX", layer: 3, parent: 10, color: "#1a2b34" },
  { id: 16, name: "jQuery", layer: 3, parent: 10, color: "#0769ad" },

  // layer 4 limit-12
  { id: 17, name: "Rollup", layer: 4, parent: 11, color: "#ec4a3f" },
  {
    id: 18,
    name: "Vite",
    query: ["{ path:vite.config.*"],
    layer: 4,
    parent: 17,
    color: "#646c70",
  },
  { id: 19, name: "Design System", layer: 4, parent: 18, color: "#ff4785" },
  { id: 20, name: "Tailwind", layer: 3, parent: 11, color: "#38b2ac" },
  {
    id: 21,
    name: "SCSS OR SASS",
    query: ["{ path:*.scss"],
    layer: 3,
    parent: 11,
    color: "#cc6699",
  },
  { id: 22, name: "PostCSS", layer: 3, parent: 11, color: "#dd3a0a" },
  { id: 23, name: "Tauri", layer: 4, parent: 10, color: "#24c8db" },

  // layer 2 limit-6
  {
    id: 24,
    name: "CSharp",
    query: ["language:csharp"],
    layer: 2,
    parent: 2,
    color: "#239120",
  },
  {
    id: 25,
    name: "Golang",
    query: ["language:go"],
    layer: 2,
    parent: 2,
    color: "#00acd7",
  },
  {
    id: 26,
    name: "Rust",
    query: ["language:rust"],
    layer: 2,
    parent: 2,
    color: "#dea584",
  },
  // { id: 27, name: 'Database', layer: 2, parent: 3, color: '#4479a1' },
  {
    id: 28,
    name: "Python",
    query: ["lanaguage:python"],
    layer: 2,
    parent: 3,
    color: "#306998",
  },

  // layer 3 limit-9
  { id: 29, name: "PostgreSQL", layer: 3, parent: 27, color: "#336791" },
  { id: 30, name: "MySQL", layer: 3, parent: 27, color: "#4479a1" },
  { id: 31, name: "MongoDB", layer: 3, parent: 27, color: "#13aa52" },
  { id: 32, name: "Redis", layer: 3, parent: 27, color: "#d82c20" },

  { id: 33, name: "Pytorch", layer: 3, parent: 28, color: "#ee4c2c" },
  { id: 34, name: "Django", layer: 3, parent: 28, color: "#092e20" },
  { id: 35, name: "Flask", layer: 3, parent: 28, color: "#306998" },

  {
    id: 36,
    name: "Docker",
    query: ["path:*docker*"],
    layer: 3,
    parent: 4,
    color: "#2496ed",
  },
  // layer 4 limit-12
  { id: 37, name: "Actrix", layer: 4, parent: 26, color: "#3386f2" },
  { id: 38, name: "Axum", layer: 4, parent: 26, color: "#00599c" },
  { id: 39, name: "Tonic", layer: 4, parent: 26, color: "#754425" },
  { id: 40, name: "Leptos", layer: 4, parent: 26, color: "#dea584" },

  { id: 41, name: "Unity", layer: 4, parent: 5, color: "#000000" },
  { id: 42, name: "Godot", layer: 4, parent: 5, color: "#478cbf" },

  { id: 43, name: "Gin", layer: 4, parent: 26, color: "#00acd7" },
  { id: 44, name: "gRPC", layer: 4, parent: 26, color: "#76a5af" },

  { id: 45, name: "ASP.NET", layer: 4, parent: 24, color: "#5c2d91" },
];

const repoNodeList = [];

type repoNode = {
  id: number;
  name: string;
  repos: Array<repoResult>;
};

type repoResult = {
  id: string;
  name: string;
  description: string;
  url: string;
  languages: any;
  license: string;
  updated_at: string;
  tags: Array<string>;
};

import { load } from "https://deno.land/std@0.224.0/dotenv/mod.ts";

const env = await load();

console.log(env);

const octokit = new Octokit({
  auth: env["GITHUB_TOKEN"],
});

const { data: fullRepoList } = await octokit.rest.search.repos({
  q: "owner:stvchm9703 -is:fork",
  per_page: 2000,
});

// console.log(fullRepoList);
let currRepoList = fullRepoList.items.map(
  (repo) =>
    ({
      id: repo.node_id,
      name: repo.full_name,
      description: repo.description,
      url: repo.html_url,
      languages: [repo.language],
      license: repo.license?.name,
      updated_at: repo.updated_at,
      tags: [],
    }) as repoResult,
);

// console.log(currRepoList);

for (let i = 0; i < currRepoList.length; i++) {
  await delay(6000);
  const repo = currRepoList[i];
  console.log(repo.name);
  const { data: languages } = await octokit.rest.repos.listLanguages({
    owner: "stvchm9703",
    repo: repo.name.split("/")[1],
  });
  currRepoList[i].languages = languages;
}

// console.log(currRepoList);

Deno.writeTextFileSync(
  "./full-repo-data.json",
  JSON.stringify(currRepoList, null, 2),
);
