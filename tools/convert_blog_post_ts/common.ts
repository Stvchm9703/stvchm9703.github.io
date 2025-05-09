import type { Tag, TagId, TagMap } from "./tag";
import type {
  Block_Content_Text_Mark,
  Block_Content_Text_Mark_Type,
  SmartBlockType,
} from "../../protos/anytype/models";

import { ObjectTypes, getShortCode } from "./enum_token";
import type { SnapshotWithType } from "../../protos/anytype/snapshot";
import { isEmpty } from "lodash-es";

// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export type DataMap = { [key: string]: any };
export const GLOBAL_RELATION_IDMAP: TagMap = new Map();
export const GLOBAL_RELATION_NAMEMAP: TagMap = new Map();
export const GLOBAL_RELATION_NAMEMAP_STR: Map<string, string> = new Map();

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

const GLOBAL_FILE_IDMAP = new Map<string, string>();
const test_id_list = new Set<string>();
const test_easy_id_list = new Set<string>();
let hitcount = 0,
  easyhitcount = 0;
export function fileIdShorten(
  sbType: SmartBlockType,
  objectTypes: ObjectTypes[],
  id: string,
  createDate: number,
  updateDate: number,
  publishDate: number,
): string {
  const easyId = id.slice(-8);
  const idList = [
    sbType.valueOf(),
    // objectType.valueOf(),
    ...objectTypes.map(getShortCode),
    easyId,
    createDate,
    updateDate === 0 ? "n" : Math.abs(updateDate - createDate),
    publishDate === 0 ? "n" : Math.abs(publishDate - createDate),
  ];
  const idTestList = idList.join("-");
  console.log(idList, idTestList);
  if (test_id_list.has(idTestList)) {
    console.warn(`Duplicate id: ${idTestList}`);
    hitcount++;
  } else {
    test_id_list.add(idTestList);
  }
  if (test_easy_id_list.has(easyId)) {
    console.warn(`Duplicate easy id: ${easyId}`);
    easyhitcount++;
  } else {
    test_easy_id_list.add(easyId);
  }

  return idTestList;
}

export function getShortenId(raw: SnapshotWithType): string {
  // return fileIdShorten(
  //   raw.sbType,
  //   raw.snapshot?.data?.objectTypes ?? [],
  //   getFieldValue<string>(raw.snapshot?.data?.details, "id") || "",
  //   getFieldValue<number>(raw.snapshot?.data?.details, "createdDate") || 0,
  //   getFieldValue<number>(raw.snapshot?.data?.details, "lastModifiedDate") || 0,
  //   getFieldValue<number>(raw.snapshot?.data?.details, "publish date") || 0,
  // );

  return (
    getFieldValue<string>(raw.snapshot?.data?.details, "id") || "00000000"
  ).slice(-8);
}

export const testIdResult = () =>
  console.log({
    hitcount,
    resolvedCount: test_id_list.size,
    easyhitcount,
    resolvedEasyCount: test_easy_id_list.size,
  });

export const pathResolver = (path: string) =>
  path
    .replace(/\s+/g, "_")
    .replace(/\W/g, "")
    .toLowerCase()
    .split("_")
    .filter((e) => !isEmpty(e))
    .join("-");

export const headerIdResolver = (text: string, id: string) =>
  `${pathResolver(text || "")}-${id.slice(-6)}`;
