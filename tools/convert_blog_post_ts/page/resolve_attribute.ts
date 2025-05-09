// import { pathResolver } from "../utils/path_resolver";
import type {
  Page,
  FuncPageAttrbuteResolver,
  FuncPageExternalResolver,
} from "./index";
import type { Tag, TagOptionLabel } from "../tag";
import {
  pathResolver,
  GLOBAL_RELATION_IDMAP,
  GLOBAL_RELATION_NAMEMAP,
  headerIdResolver,
} from "../common";

let _seriesRel: Tag | null = null;

export const resolveSeries: FuncPageAttrbuteResolver = (self: Page): Page => {
  if (self.serie === null || self.serie.length === 0) return self;

  if (_seriesRel == null) {
    _seriesRel = GLOBAL_RELATION_IDMAP.values().find(
      (elm) => elm.name === "Series",
    );
  }
  // console.log(self.serie);

  const resolved = self.serie
    .map((elm) => {
      const tagObj = _seriesRel.options.find((e) => e.id === elm);
      return tagObj
        ? ({
            id: tagObj.id,
            name: tagObj.name,
            _sid: tagObj._sid,
            url: `/posts/series/${pathResolver(tagObj.name)}_${tagObj._sid}`,
          } as TagOptionLabel)
        : elm;
    })
    .filter((elm) => elm !== null);

  // self.serie = { ...resolved[0] };
  if (resolved.length > 0) {
    self.serie = resolved[0];
  } else {
    self.serie = null;
  }
  // self.serie = { ...resolved[0] };
  return self;
};

export const resolveTags: FuncPageExternalResolver = (
  self: Page,
  rawTags: Tag,
): Page => {
  // console.log(rawTags);
  const resolved = self.raw_tag_list
    .map((elm) => {
      const tagObj = rawTags.options.find((e) => e.id === elm);
      return tagObj
        ? ({
            id: tagObj.id,
            _sid: tagObj._sid,
            name: tagObj.name,
            url: `/posts/tags/${pathResolver(tagObj.name)}_${tagObj._sid}`,
          } as TagOptionLabel)
        : null;
    })
    .filter((elm) => elm !== null);
  self.tags = resolved;
  return self;
};

export const resolveTOCComponent: FuncPageAttrbuteResolver = (
  self: Page,
): Page => {
  const resolved = self.reformedContents
    .filter(
      (elm) =>
        elm.componentType === "Text" &&
        elm.componentAttr["style"]?.startsWith("Header"),
    )
    .map((elm) => {
      const title = elm.componentAttr["text"].toString();
      const eid = headerIdResolver(title, elm.id);

      return {
        id: eid,
        componentId: elm.id,
        title,
        url: `#${eid}`,
        level: Number.parseInt(
          elm.componentAttr["style"]?.replace("Header", ""),
        ),
      } as PageLink;
    })
    .filter((elm) => elm !== null);

  // console.log(resolved);

  self.tableOfContents = resolved;
  return self;
};
