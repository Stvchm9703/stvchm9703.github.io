import type { Tag, TagId, TagMap } from "./tag";
import type {
  Block_Content_Text_Mark,
  Block_Content_Text_Mark_Type,
} from "../../protos/anytype/models";

// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export type DataMap = { [key: string]: any };
export let GLOBAL_RELATION_IDMAP: TagMap = new Map();
export let GLOBAL_RELATION_NAMEMAP: TagMap = new Map();
export let GLOBAL_RELATION_NAMEMAP_STR: Map<string, string> = new Map();

// export type OutputAttributes = {
//   [key: string]: any;
// };

export function getFieldValue<T>(
  dataMap: DataMap | undefined,
  inputKey: string,
): T | null {
  if (!dataMap) {
    console.error("DataMap is undefined");
    throw new Error("DataMap is undefined");
  }
  const value = dataMap[inputKey];
  if (value) return value as T;

  //try from GLOBAL_RELATION_NAMEMAP_STR
  console.warn("try from custom key");

  let customKey = GLOBAL_RELATION_NAMEMAP_STR.get(inputKey);
  // console.warn({ inputKey, customKey, field: dataMap[customKey ?? ""] });

  if (customKey && dataMap[customKey]) return dataMap[customKey] as T;

  console.warn(`Field '${inputKey}' not found`);
  console.log(
    ` "hint: check the field name list and fix the field name : ",${Object.keys(dataMap)}' not found`,
  );
  // throw new Error(`Field '${key}' not found`);
  return null;
}

export function setRelationNameMap(tag: Tag): void {
  GLOBAL_RELATION_IDMAP.set(tag.id, tag);
  GLOBAL_RELATION_NAMEMAP.set(tag.relationKey, tag);
  GLOBAL_RELATION_NAMEMAP_STR.set(tag.name, tag.relationKey);
}
// export type TextMark =
export interface TextMark {
  /** range of symbols to apply this mark. From(symbol) To(symbol) */
  range: Range | undefined;
  type: TextMarkType | string | number;
  /** link, color, etc */
  param: string;
}
export type TextMarkType = Block_Content_Text_Mark_Type;
