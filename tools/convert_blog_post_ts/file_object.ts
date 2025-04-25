// import { stack } from "d3";
import type { Block as PbBlock } from "../../protos/anytype/models";
import { SmartBlockType } from "../../protos/anytype/models";

import { SnapshotWithType } from "../../protos/anytype/snapshot";
import { castToAttributeMap, type AttributeMap } from "./attribute";
// import type { Collection, CollectionId } from "./collection";
import { getFieldValue, GLOBAL_RELATION_IDMAP } from "./common";
import type { DataMap } from "./common";
// import type { ContentBlockList, ContentBlockMap } from "./content_block";
// import * as ContentBlock from "./content_block";
import { ObjectTypes } from "./enum_token";
// import { ExternalBookmarkLink } from "./external_link";
// import type { TagList, TagMap, Tag, TagOptionLabel } from "./tag";
// import type { UserId } from "./user";
// import type { WorkspaceId } from "./workspace";
// import { last } from "lodash-es";

export interface FileObject {
  id: string;
  title: string;
  fileUrl: string;
  fileExt: string;
  attributes: DataMap;
}

export const getFileObjects = (
  snapshot_list: SnapshotWithType[],
): FileObject[] => {
  return snapshot_list
    .filter(
      (snapshot) =>
        snapshot.sbType === SmartBlockType.File ||
        snapshot.sbType === SmartBlockType.FileObject,
    )
    .map(fromAnytype);
};

function fromAnytype(raw: SnapshotWithType): FileObject {
  const data = raw.snapshot?.data;
  const detailMap = data?.details;

  const tmp: FileObject = {
    id: getFieldValue(detailMap, "id") ?? "",
    title: getFieldValue(detailMap, "name") ?? "",
    attributes: detailMap ?? {},
    fileUrl: getFieldValue(detailMap, "source") ?? "",
    fileExt: getFieldValue(detailMap, "fileExt") ?? "",
  };

  return tmp;
}
