// import Fuse from 'https://deno.land/x/fuse/dist/fuse.esm.js'

// import { nodes } from './data.js'

// import repoList from '../../assets/content/gh-repo-all.json'

import { Octokit } from "https://esm.sh/@octokit/rest";
import { createTokenAuth } from "https://esm.sh/@octokit/auth-token";
import { delay } from "https://deno.land/x/delay@v0.2.0/mod.ts";


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


const old_list_txt = await Deno.readTextFile("./full-repo-data.json");
const old_list: Array<repoResult> = JSON.parse(old_list_txt);

const new_list_txt = await Deno.readTextFile("./full-repo-data_new.json")
const new_list: Array<repoResult> = JSON.parse(new_list_txt);

for (const item of new_list) {
  const same_item = old_list.find(e => e.id === item.id);
  if (same_item && same_item.tags !== undefined) {
    item.tags = same_item.tags;
  }
  // console.log(`${item.name} [${item.tags}]`)
}

const remain_check = new_list.filter(e => e.tags === undefined || e.tags.length === 0);
for (const item of remain_check) {
  console.log(`${item.name} [${item.tags}]`)
}

// overwrite
Deno.writeTextFileSync(
  "./full-repo-data_old.json",
  old_list_txt
)

Deno.writeTextFileSync(
  "./full-repo-data.json",
  JSON.stringify(new_list, null, 2)
);

Deno.writeTextFileSync(
  "./check-list.txt",
  remain_check.map(elem => `${elem.id}  ${elem.name} \n`)
);