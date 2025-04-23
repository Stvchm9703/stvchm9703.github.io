import * as PbModels from "../../protos/anytype/models";
import * as PbSnapshot from "../../protos/anytype/snapshot";

import { Glob } from "bun";
import { ObjectTypes } from "./enum_token";
import * as Workspace from "./workspace";
import * as Collection from "./collection";
import * as Page from "./page";
import * as Tag from "./tag";

import { intersectionBy, intersection } from "lodash-es";

import {
  GLOBAL_RELATION_NAMEMAP,
  GLOBAL_RELATION_IDMAP,
  setRelationNameMap,
} from "./common";
import { getBookmarkLinks } from "./external_link";

const ANYTYPE_EXPORT_PATH = "blog_post/Anytype.20250423.162011.34/**/*.pb";

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
  const rawPages = Page.getPages(snapshot_list);
  const rawBookmarks = getBookmarkLinks(snapshot_list);

  // article collection
  const article_coll = collections.find((c) => c.name === "Article");
  if (!article_coll) {
    console.warn("Article collection not found");
    return;
  }
  Collection.insertArticles(article_coll, rawPages);

  // let debug = "";
  const defaultTag = taglist.find((tag) => tag.name === "Tag");
  // console.log(GLOBAL_RELATION_IDMAP);
  for (const pageId in article_coll.articles) {
    let page = article_coll.articles[pageId];
    if (defaultTag) {
      Page.resolveTags(page, defaultTag);
      Page.resolveSeries(page);
    }

    Page.resolveLinkComponent(page, rawBookmarks);

    // // page.raw_tag_list = undefined;
    // delete page.raw_tag_list;
    // delete page.contents;
  }

  for (const pageId in article_coll.articles) {
    let page = article_coll.articles[pageId];
    let relatedChapters: Page.PageLink[] = [];
    let relatedArticles: Page.PageLink[] = [];
    for (const subPageId in article_coll.articles) {
      const elm = article_coll.articles[subPageId];
      if (elm.serie?.id === page.serie?.id && elm.id !== page.id) {
        relatedChapters.push({
          id: elm.id,
          componentId: elm.id,
          title: elm.title,
          url: `/posts/${elm.id}_${pathResolver(elm.title)}`,
        }) as Page.PageLink;
      }
      // console.log(elm.raw_tag_list, page.raw_tag_list);

      if (
        intersection(elm.raw_tag_list, page.raw_tag_list).length > 0 &&
        elm.id !== page.id
      ) {
        relatedArticles.push({
          id: elm.id,
          componentId: elm.id,
          title: elm.title,
          url: `/posts/${elm.id}_${pathResolver(elm.title)}`,
        }) as Page.PageLink;
      }
    }

    page.relatedChapters = relatedChapters;
    page.relatedArticles = relatedArticles;
  }

  const output_file_list = [];

  for (const pageId in article_coll.articles) {
    let page = article_coll.articles[pageId];
    let page_content_path = `blog_post_resolved/${pathResolver(article_coll.name)}_${article_coll.id.slice(-6)}/${pageId}.json`;

    output_file_list.push({
      post_id: page.id,
      res: pathResolver(page.title),
      page_content_path,
    });

    Bun.write(page_content_path, JSON.stringify(page, null, 2));
  }
  Bun.write(
    "blog_post_resolved/index.json",
    JSON.stringify(output_file_list, null, 2),
  );

  // console.log(article_coll.articles.values()[0].contents);
  // console.log(GLOBAL_RELATIONSHIP_NAMEMAP.get("tag"));

  // console.log(GLOBAL_RELATIONSHIP_NAMEMAP);
  // console.log({ publishDate: tags.filter((e) => e.name.includes("publish")) });
  // debug;
};

const pathResolver = (path: string) =>
  path.replace(/\s+/g, "_").replace(/\W/g, "").toLowerCase();

main();
