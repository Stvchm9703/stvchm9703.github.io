// import { stack } from "d3";
import type { Block as PbBlock } from "../../protos/anytype/models";
import { SmartBlockType } from "../../protos/anytype/models";

import { SnapshotWithType } from "../../protos/anytype/snapshot";
import { castToAttributeMap, type AttributeMap } from "./attribute";
// import type { Collection, CollectionId } from "./collection";
import { getFieldValue, getShortenId, GLOBAL_RELATION_IDMAP } from "./common";
import type { DataMap } from "./common";
// import type { ContentBlockList, ContentBlockMap } from "./content_block";
// import * as ContentBlock from "./content_block";
import { ObjectTypes } from "./enum_token";
// import { ExternalBookmarkLink } from "./external_link";
// import type { TagList, TagMap, Tag, TagOptionLabel } from "./tag";
// import type { UserId } from "./user";
// import type { WorkspaceId } from "./workspace";
// import { last } from "lodash-es";
//

export interface FileObject {
  id: string;
  _sid: string;
  title: string;
  fileUrl: string;
  fileExt: string;
  fileType: string;
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
  const fileExt = getFieldValue<string>(detailMap, "fileExt") ?? "";
  const fileMimeType = getFieldValue<string>(detailMap, "fileMimeType") ?? "";

  const tmp: FileObject = {
    id: getFieldValue(detailMap, "id") ?? "",
    _sid: getShortenId(raw),
    title: getFieldValue(detailMap, "name") ?? "",
    attributes: detailMap ?? {},
    fileUrl: getFieldValue(detailMap, "source") ?? "",
    fileExt,
    fileType: resolveFileType(fileExt, fileMimeType),
  };

  return tmp;
}

const resolveFileType = (fileExt: string, fileMimeType: string): string => {
  if (fileMimeType.startsWith("image")) return "images";
  if (fileMimeType.startsWith("video")) return "videos";
  if (fileMimeType.startsWith("audio")) return "audios";
  // if (fileMimeType.startsWith('document')) return "Document";
  if (fileExt === "pdf") return "pdf";
  if (
    fileExt === "doc" ||
    fileExt === "docx" ||
    fileExt === "xls" ||
    fileExt === "xlsx" ||
    fileExt === "ppt" ||
    fileExt === "pptx"
  )
    return "documents";

  if (fileExt === "txt") return "text";

  if (fileExt === "csv" || fileExt === "json" || fileExt === "xml")
    return "data";

  if (fileExt === "ipynb") return "notebook";

  return "raw";
};

export const exportForMetaData = (fileList: FileObject[]) => {
  const images = fileList
    .filter((f) => f.fileType === "images")
    .map(
      (e) =>
        ({
          url: `/blog/assets/${e.fileUrl}`,
          alt: e.title,
          height: e.attributes["heightInPixels"],
          width: e.attributes["widthInPixels"],
          type: e.attributes["fileMimeType"],
        }) as OpenGraphImage,
    );

  const videos = fileList
    .filter((f) => f.fileType === "videos")
    .map(
      (e) =>
        ({
          url: `/blog/assets/${e.fileUrl}`,
          type: e.attributes["fileMimeType"],
          height: e.attributes["heightInPixels"],
          width: e.attributes["widthInPixels"],
        }) as OpenGraphVideos,
    );
  const audios = fileList
    .filter((f) => f.fileType === "audios")
    .map(
      (e) =>
        ({
          url: `/blog/assets/${e.fileUrl}`,
          type: e.attributes["fileMimeType"],
        }) as OpenGraphAudio,
    );

  let result = {};
  if (images.length > 0) {
    // result.images = images;
    result = Object.assign(result, { images });
  }
  if (videos.length > 0) {
    result = Object.assign(result, { videos });
  }
  if (audios.length > 0) {
    result = Object.assign(result, { audios });
  }

  return result;
};

export interface OpenGraphImage {
  url: string;
  secureUrl?: string;
  type?: string;
  width?: number;
  height?: number;
  alt?: string;
}

export interface OpenGraphVideos {
  url: string;
  secureUrl?: string;
  type?: string;
  width?: number;
  height?: number;
}

export interface OpenGraphAudio {
  url: string;
  secureUrl?: string;
  type?: string;
}
