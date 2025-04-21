// import { Result, Error } from 'some-error-library'; // Adjust the import based on your error handling library

import { SmartBlockType } from "../../protos/anytype/models";
import type { SnapshotWithType } from "../../protos/anytype/snapshot";
import { castToAttributeMap, type AttributeMap } from "./attribute";
import { getFieldValue } from "./common";

export type TagId = string;
export type TagList = Tag[];
export type TagMap = Map<TagId, Tag>;

export interface Tag {
  id: TagId;
  name: string;
  description: string;
  relationKey: string; // st-relationshipKey
  // styles: string[];
  attributes: AttributeMap; // Define AttributeMap according to your needs
  options: TagOption[];
}
export interface TagOption {
  id: TagId;
  name: string;
  description: string;
  relationKey: string;
  // styles: string[];
  attributes: AttributeMap; // Define AttributeMap according to your needs
  // options: [];
}
export interface TagOptionLabel {
  id: TagId;
  name: string;
}

export const getTags = (snapshot_list: SnapshotWithType[]): Tag[] => {
  const tag_opt = snapshot_list
    .filter((snapshot) => snapshot.sbType === SmartBlockType.STRelationOption)
    .map(fromAnytypeOption);

  return snapshot_list
    .filter((snapshot) => snapshot.sbType === SmartBlockType.STRelation)
    .map(fromAnytype)
    .map((tag) => ({
      ...tag,
      options: tag_opt.filter((opt) => opt.relationKey === tag.relationKey),
    }));
};

export function fromAnytype(raw: SnapshotWithType): Tag {
  const tmp: Tag = {
    id: "",
    name: "",
    description: "",
    relationKey: "", // relation-ship
    // styles: [],
    attributes: {} as AttributeMap, // Initialize according to your needs

    options: [],
  };

  const dataMap = raw.snapshot?.data?.details;

  if (!dataMap) {
    throw new Error("Data map is undefined");
  }

  tmp.id = getFieldValue(dataMap, "id") ?? "";
  tmp.name = getFieldValue(dataMap, "name") ?? "";
  tmp.description = getFieldValue(dataMap, "description") ?? "";
  tmp.relationKey = getFieldValue(dataMap, "relationKey") ?? "";
  tmp.attributes = castToAttributeMap(dataMap);

  return tmp;
}

export function fromAnytypeOption(raw: SnapshotWithType): TagOption {
  const tmp: TagOption = {
    id: "",
    name: "",
    description: "",
    relationKey: "", // relation-ship
    attributes: {} as AttributeMap, // Initialize according to your needs
  };

  const dataMap = raw.snapshot?.data?.details;

  if (!dataMap) {
    throw new Error("Data map is undefined");
  }

  tmp.id = getFieldValue(dataMap, "id") ?? "";
  tmp.name = getFieldValue(dataMap, "name") ?? "";
  // tmp.description = getFieldValue(dataMap, "description") ?? "";
  tmp.relationKey = getFieldValue(dataMap, "relationKey") ?? "";
  tmp.attributes = castToAttributeMap(dataMap);

  return tmp;
}
