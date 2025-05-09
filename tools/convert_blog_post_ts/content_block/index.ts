import { castToAttributeMap, type AttributeMap } from "../attribute";
import type {
  Block_Content_Smartblock,
  Block_Content_Text,
  Block_Content_File,
  Block_Content_Layout,
  Block_Content_Div,
  Block_Content_Bookmark,
  Block_Content_Icon,
  Block_Content_Link,
  Block_Content_Dataview,
  Block_Content_Relation,
  Block_Content_FeaturedRelations,
  Block_Content_Latex,
  Block_Content_TableOfContents,
  Block_Content_Table,
  Block_Content_TableColumn,
  Block_Content_TableRow,
  Block_Content_Widget,
  Block_Content_Chat,
} from "../../../protos/anytype/models";

import { Block } from "../../../protos/anytype/models";

import {
  Block_Align,
  Block_VerticalAlign,
} from "../../../protos/anytype/models";
import type { DataMap, TextMark } from "../common";

import { resolveFileComponent } from "./file";
import { resolveLayoutRowComponent } from "./flexbox";
import { resolveTableComponent } from "./table";
import { resolveTextNumberComponent } from "./text_number";
import { resolveTextToggleComponent } from "./text_toggle";

export {
  resolveFileComponent,
  resolveLayoutRowComponent,
  resolveTableComponent,
  resolveTextNumberComponent,
  resolveTextToggleComponent,
};

export type ContentBlockMap = Map<string, ContentBlock>;

export type ContentBlockList = Array<ContentBlock>;

export interface ContentBlock {
  id: string;
  order: number;
  fields: DataMap;
  // restrictions: Block_Restrictions | undefined;
  childrenIds: string[];
  /// base layout style
  style: ContentBlockBasicStyle;
  // backgroundColor: string;
  // align: Block_Align;
  // verticalAlign: Block_VerticalAlign;
  // extensional

  componentType: string;
  componentAttr?: DataMap;
}

export type ContentBlockComponent =
  | Block_Content_Smartblock
  | Block_Content_Text
  | Block_Content_File
  | Block_Content_Layout
  | Block_Content_Div
  | Block_Content_Bookmark
  | Block_Content_Icon
  | Block_Content_Link
  | Block_Content_Dataview
  | Block_Content_Relation
  | Block_Content_FeaturedRelations
  | Block_Content_Latex
  | Block_Content_TableOfContents
  | Block_Content_Table
  | Block_Content_TableColumn
  | Block_Content_TableRow
  | Block_Content_Widget
  | Block_Content_Chat;

export function fromBlock(idx: number, raw: Block): ContentBlock {
  // const { type, attributes } = getComponentMap(raw);

  // coinst blockdd = B
  // console.log(raw.fields);
  const cblk: ContentBlock = {
    id: raw.id,
    order: idx,
    // fields: castToAttributeMap(raw.fields),
    fields: raw.fields ?? {},
    childrenIds: raw.childrenIds,
    style: convertToStyle(raw),
    componentAttr: getComponentMap(raw),
    componentType: getComponentTypeString(raw),
  };

  return cblk;
}

export interface ContentBlockBasicStyle {
  backgroundColor: string;
  align: string;
  verticalAlign: string;
}

function convertToStyle(raw: Block): ContentBlockBasicStyle {
  const style: ContentBlockBasicStyle = {
    backgroundColor: raw.backgroundColor || "",
    align: Block_Align[raw.align || Block_Align.AlignLeft].toString(),
    verticalAlign:
      Block_VerticalAlign[
        raw.verticalAlign || Block_VerticalAlign.VerticalAlignTop
      ].toString(),
  };
  return style;
}

export function getComponentTypeString(rawBlock: Block): string {
  // if (!rawBlock) return "Unknown";
  if (rawBlock.smartblock !== undefined) return "Smartblock";
  if (rawBlock.text !== undefined) return "Text";
  if (rawBlock.file !== undefined) return "File";
  if (rawBlock.layout !== undefined) return "Layout";
  if (rawBlock.div !== undefined) return "Div";
  if (rawBlock.bookmark !== undefined) return "Bookmark";
  if (rawBlock.icon !== undefined) return "Icon";
  if (rawBlock.link !== undefined) return "Link";
  if (rawBlock.dataview !== undefined) return "Dataview";
  if (rawBlock.relation !== undefined) return "Relation";
  if (rawBlock.featuredRelations !== undefined) return "FeaturedRelations";
  if (rawBlock.latex !== undefined) return "Latex";
  if (rawBlock.tableOfContents !== undefined) return "TableOfContents";
  if (rawBlock.table !== undefined) return "Table";
  if (rawBlock.tableColumn !== undefined) return "TableColumn";
  if (rawBlock.tableRow !== undefined) return "TableRow";
  if (rawBlock.widget !== undefined) return "Widget";
  if (rawBlock.chat !== undefined) return "Chat";

  return "Unknown";
}

function getComponentMap(rawBlock: Block): DataMap {
  let typeKey = getComponentTypeString(rawBlock);
  typeKey = typeKey[0].toLowerCase() + typeKey.slice(1);
  const ctx = Block.toJSON(rawBlock);
  return ctx[typeKey] as DataMap;
}

export type FuncContentBlockResolver = (
  blockMap: ContentBlockMap,
  ...args: any[]
) => ContentBlockMap;
