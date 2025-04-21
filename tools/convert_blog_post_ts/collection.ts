// import { BTreeMap } from "immutable"; // Assuming you are using Immutable.js for BTreeMap
// import { AnytypeObject, SnapshotWithType, Error } from "./types"; // Adjust the import according to your types
import { castToAttributeMap, type AttributeMap } from "./attribute";
import type { SnapshotWithType } from "../../protos/anytype/snapshot";
import type { WorkspaceId } from "./workspace";
import type { Page, PageMap } from "./page";
import { getFieldValue } from "./common";
import { ObjectTypes } from "./enum_token";
export type CollectionId = string;
export type CollectionMap = Map<CollectionId, Collection>;

// object interface
export interface Collection {
  id: CollectionId;
  name: string;
  description: string;
  cover: string;
  styles: string[];
  attributes: AttributeMap;
  workspace_id: WorkspaceId;
  article_id_list: string[];
  articles: PageMap;

  // fromAnytype?(raw: SnapshotWithType): Collection;
}

export function fromAnytype(raw: SnapshotWithType): Collection {
  const dataMap = raw.snapshot?.data?.details;

  const collection: Collection = {
    id: getFieldValue<string>(dataMap, "id"),
    name: getFieldValue<string>(dataMap, "name"),
    // attributes: getFieldValue<AttributeMap>(dataMap, "attributes"),
    attributes: castToAttributeMap(dataMap),
    article_id_list: getFieldValue<string[]>(dataMap, "links"),
    workspace_id: getFieldValue(dataMap, "spaceId"),
    description: "", // Assuming default value
    cover: "", // Assuming default value
    styles: [], // Assuming default value
    articles: {} as PageMap,
  };

  return collection;
}

export function insertArticles(collection: Collection, files: Page[]): void {
  for (const key of collection.article_id_list) {
    const file = files.find((f) => f.id === key);
    if (file) {
      collection.articles[file.id] = file; // Assuming articles is an object
    }
  }
}

export const getCollections = (
  snapshot_list: SnapshotWithType[],
): Collection[] => {
  return snapshot_list
    .filter((snapshot) =>
      snapshot.snapshot?.data?.objectTypes.includes(
        ObjectTypes.Collection.toString(),
      ),
    )
    .map(fromAnytype);
};
