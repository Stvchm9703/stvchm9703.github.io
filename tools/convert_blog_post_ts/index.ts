import * as PbModels from "../../protos/anytype/models";
import * as PbSnapshot from "../../protos/anytype/snapshot";

import { Glob } from "bun";
import { ObjectTypes } from "./enum_token";
import * as Workspace from "./workspace";
import * as Collection from "./collection";
import * as Page from "./page";
import * as Tag from "./tag";

import {
  GLOBAL_RELATION_NAMEMAP,
  GLOBAL_RELATION_IDMAP,
  setRelationNameMap,
} from "./common";

const ANYTYPE_EXPORT_PATH = "blog_post/Anytype.20250418.013727.47/**/*.pb";

const main = async () => {
  const glob = new Glob(ANYTYPE_EXPORT_PATH);
  let snapshot_list: PbSnapshot.SnapshotWithType[] = [];
  for await (const file of glob.scan()) {
    // console.log(`looking in <${file}>`);
    const file_byte = Bun.file(file);

    const snapshot = PbSnapshot.SnapshotWithType.decode(
      await file_byte.bytes(),
    );

    snapshot_list.push(snapshot);
  }
  // Bun.file();
  // console.log(snapshot_list[0].snapshot?.data?.details);

  resolveCollection(snapshot_list);
};

const resolveCollection = async (
  snapshot_list: PbSnapshot.SnapshotWithType[],
) => {
  const taglist = Tag.getTags(snapshot_list);
  taglist.forEach(setRelationNameMap);
  const workspace = Workspace.getWorkspaces(snapshot_list);
  const collections = Collection.getCollections(snapshot_list);
  const pages = Page.getPages(snapshot_list);

  // article collection
  const article_coll = collections.find((c) => c.name === "Article");
  if (!article_coll) {
    console.warn("Article collection not found");
    return;
  }
  Collection.insertArticles(article_coll, pages);

  // let debug = "";
  const defaultTag = taglist.find((tag) => tag.name === "Tag");
  // console.log(GLOBAL_RELATION_IDMAP);
  for (const pageId in article_coll.articles) {
    const page = article_coll.articles[pageId];
    if (defaultTag) Page.resolveTags(page, defaultTag);

    // page.raw_tag_list = undefined;
    delete page.raw_tag_list;
    delete page.contents;

    if (
      pageId == "bafyreiaaezcucbcokt5vw4k4lp6qsf6r6hkqufx4hha27x5z4g5k5ggpfa"
    ) {
      // console.log(page);
      // debug = JSON.stringify(page);
      Bun.write(`${pageId}.json`, JSON.stringify(page, null, 2));
    }
    // page.
  }
  // console.log(article_coll.articles.values()[0].contents);
  // console.log(GLOBAL_RELATIONSHIP_NAMEMAP.get("tag"));

  // console.log(GLOBAL_RELATIONSHIP_NAMEMAP);
  // console.log({ publishDate: tags.filter((e) => e.name.includes("publish")) });
  // debug;
};

main();
