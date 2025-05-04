import * as PbModels from "../../protos/anytype/models";
import * as PbSnapshot from "../../protos/anytype/snapshot";

import { Glob } from "bun";
import { ObjectTypes } from "./enum_token";
import * as Workspace from "./workspace";
import * as Collection from "./collection";
import * as Page from "./page";
import * as Tag from "./tag";
import * as Jupyter from "./jupyter";

import { intersectionBy, intersection, isEmpty, chunk } from "lodash-es";

import {
  GLOBAL_RELATION_NAMEMAP,
  GLOBAL_RELATION_IDMAP,
  setRelationNameMap,
  testIdResult,
  pathResolver,
} from "./common";
import { getBookmarkLinks } from "./external_link";
import { parseArgs } from "util";
import { getFileObjects } from "./file_object";

// const ANYTYPE_EXPORT_PATH = "blog_post/Anytype.20250424.095547.72/**/*.pb";

const main = async () => {
  const { values, positionals } = parseArgs({
    args: Bun.argv,
    options: {
      importPath: {
        type: "string",
      },
    },
    strict: true,
    allowPositionals: true,
  });
  const importPath = `${values["importPath"]}/**/*.pb`;

  const glob = new Glob(importPath);
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

  resolveCollection(snapshot_list, values["importPath"]);
};

const resolveCollection = async (
  snapshot_list: PbSnapshot.SnapshotWithType[],
  imoprtPath: string,
) => {
  const taglist = Tag.getTags(snapshot_list);
  taglist.forEach(setRelationNameMap);

  const workspace = Workspace.getWorkspaces(snapshot_list);
  const collections = Collection.getCollections(snapshot_list);
  const rawPages = Page.getPages(snapshot_list);
  const rawBookmarks = getBookmarkLinks(snapshot_list);

  const rawFile = getFileObjects(snapshot_list);
  // console.log(rawFile);
  const jupytorFile: Jupyter.JupyterNotebookRoot[] = [];

  for (const fileobj of rawFile) {
    const file = Bun.file(`${imoprtPath}/${fileobj.fileUrl}`);
    // console.log(fileobj);
    // copy for asset snapshot
    await Bun.write(`blog_post_resolved/${fileobj.fileUrl}`, file);
    // copy for static serve
    // if(fileobj.attributes[''])
    await Bun.write(`static/blog/assets/${fileobj.fileUrl}`, file);

    if (fileobj.fileUrl.endsWith(".ipynb")) {
      const nb_content = await file.text();
      const notebook = Jupyter.readJupyterNotebook(nb_content);
      notebook.fileUrl = fileobj.fileUrl;
      jupytorFile.push(notebook);
    }
  }
  // article collection
  const article_coll = collections.find((c) => c.name === "Article");
  if (!article_coll) {
    console.warn("Article collection not found");
    return;
  }
  Collection.insertArticles(article_coll, rawPages);

  const defaultTag = taglist.find((tag) => tag.name === "Tag");
  for (const pageId in article_coll.articles) {
    let page = article_coll.articles[pageId];
    if (defaultTag) {
      Page.resolveTags(page, defaultTag);
      Page.resolveSeries(page);
    }

    Page.resolveLinkComponent(page, rawBookmarks);
    Page.resolveJupyterComponent(page, jupytorFile);
    Page.resolveFileComponent(page, rawFile);
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
          _sid: elm._sid,
          componentId: elm.id,
          title: elm.title,
          url: `/posts/${elm._sid}_${pathResolver(elm.title)}`,
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
          url: `/posts/${elm._sid}_${pathResolver(elm.title)}`,
        }) as Page.PageLink;
      }
    }

    page.relatedChapters = relatedChapters;
    page.relatedArticles = relatedArticles;
  }

  let output_file_list = [];

  for (const pageId in article_coll.articles) {
    let page = article_coll.articles[pageId];
    let page_content_path = `blog_post_resolved/${pathResolver(article_coll.name)}_${article_coll.id.slice(-6)}/${pageId}.json`;

    output_file_list.push({
      post_id: page._sid,
      id: page.id,
      _sid: page._sid,
      res: pathResolver(page.title),
      page_content_path,
      publish_date: page.publish_date,
    });

    await Bun.write(page_content_path, JSON.stringify(page, null, 2));
  }

  output_file_list = output_file_list.sort((a, b) => a - b);
  await Bun.write(
    "blog_post_resolved/index.json",
    JSON.stringify(output_file_list, null, 2),
  );

  // series index to articles
  await exportSeriesIndex(article_coll);

  // tags index to articles
  await exportTagsIndex(article_coll, defaultTag);

  // testIdResult();
};

async function exportSeriesIndex(article_coll: Collection.Collection) {
  const seriesSet = GLOBAL_RELATION_IDMAP.values().find(
    (elm) => elm.name === "Series",
  );

  const allPageLink: Page.PageLink[] = [];
  for (const pageId in article_coll.articles) {
    const page: Page.Page = article_coll.articles[pageId];
    allPageLink.push({
      id: page.id,
      _sid: page._sid,
      title: page.title,
      componentId: "",
      url: `/posts/${page._sid}_${pathResolver(page.title)}`,
      level: 0,
      coverImage: page.coverImage,
      publish_date: page.publish_date,
      serie: page.serie,
      snippet: page.snippet,
      tags: page.tags,
    } as Page.PageLink);
  }
  const seriesIndexList: any[] = [];

  for (const serieOption of seriesSet?.options || []) {
    let resultList: Page.PageLink[] = allPageLink.filter(
      (page) => page.serie?.id === serieOption.id,
    );

    resultList = resultList.sort((a, b) => a.publish_date - b.publish_date);

    seriesIndexList.push({
      id: serieOption.id,
      _sid: serieOption._sid,
      name: serieOption.name,
      description: serieOption.description,
      totalCount: resultList.length,
      resultList: resultList.slice(0, 3),
    });

    const pagedResult = chunk(resultList, 150);
    for (const idx in pagedResult) {
      const pageSet = {
        id: serieOption.id,
        _sid: serieOption._sid,
        name: serieOption.name,
        description: serieOption.description,
        totalCount: resultList.length,
        totalPageNumber: pagedResult.length,
        pageIndex: Number.parseInt(idx),
        resultList: pagedResult[idx],
      };
      await Bun.write(
        `blog_post_resolved/series/${serieOption.id}/p${idx}.json`,
        JSON.stringify(pageSet, null, 2),
      );
    }
  }

  const untaggedSerie: Page.PageLink[] = allPageLink.filter((page) =>
    isEmpty(page.serie),
  );
  const untaggedSeriePageset = chunk(untaggedSerie, 150);
  for (const idx in untaggedSeriePageset) {
    const pageSet = {
      id: "-",
      _sid: "others",
      name: "Others",
      description: "uncategorized, but still valuable",
      totalCount: untaggedSerie.length,
      totalPageNumber: untaggedSeriePageset.length,
      pageIndex: Number.parseInt(idx),
      resultList: untaggedSeriePageset[idx],
    };
    await Bun.write(
      `blog_post_resolved/series/others/p${idx}.json`,
      JSON.stringify(pageSet, null, 2),
    );
  }

  seriesIndexList.push({
    id: "-",
    _sid: "others",
    name: "Others",
    description: "",
    totalCount: untaggedSerie.length,
    resultList: untaggedSerie.slice(0, 3),
  });

  const latestUpdated = allPageLink
    .sort((a, b) => a.publish_date - b.publish_date)
    .slice(0, 3);

  seriesIndexList.push({
    id: "latestUpdated",
    _sid: "latest",
    name: "Latest Updated",
    description: "",
    totalCount: allPageLink.length,
    resultList: latestUpdated,
  });

  await Bun.write(
    "blog_post_resolved/series/index.json",
    JSON.stringify(seriesIndexList, null, 2),
  );
}

async function exportTagsIndex(
  article_coll: Collection.Collection,
  defaultTag?: Tag.Tag,
) {
  let tagIndexList: any[] = [];
  for (const tagOption of defaultTag?.options || []) {
    let resultList: Page.PageLink[] = [];
    for (const pageId in article_coll.articles) {
      const page: Page.Page = article_coll.articles[pageId];
      if (page.tags.findIndex((p) => p.id === tagOption.id) !== -1) {
        resultList.push({
          id: page.id,
          title: page.title,
          componentId: "",
          url: `/posts/${page._sid}_${pathResolver(page.title)}`,
          level: 0,
          coverImage: page.coverImage,
          publish_date: page.publish_date,
          serie: page.serie,
          snippet: page.snippet,
          tags: page.tags,
        } as Page.PageLink);
      }
    }
    resultList = resultList.sort((a, b) => a.publish_date - b.publish_date);
    tagIndexList.push({
      id: tagOption.id,
      _sid: tagOption._sid,
      name: tagOption.name,
      description: tagOption.description,
      totalCount: resultList.length,
      resultList: resultList.slice(0, 3),
    });

    const pagedResult = chunk(resultList, 200);
    for (const idx in pagedResult) {
      Bun.write(
        `blog_post_resolved/tags/${tagOption.id}/p${idx}.json`,
        JSON.stringify({
          id: tagOption.id,
          _sid: tagOption._sid,
          name: tagOption.name,
          description: tagOption.description,
          totalCount: resultList.length,
          totalPageNumber: pagedResult.length,
          pageIndex: Number.parseInt(idx),
          resultList: pagedResult[idx],
          // resultList: resultList.slice(0, 3),
        }),
      );
    }
  }

  await Bun.write(
    "blog_post_resolved/tags/index.json",
    JSON.stringify(tagIndexList, null, 2),
  );
}

main();
