import type { SnapshotWithType } from "../../protos/anytype/snapshot";
import { getShortenId } from "./common";
import { ObjectTypes } from "./enum_token";

export interface ExternalBookmarkLink {
  id: string;
  _sid: string;
  href: string;
  title: string;
  level?: number;
}

export function getBookmarkLinks(
  snapshot_list: SnapshotWithType[],
): ExternalBookmarkLink[] {
  return snapshot_list
    .filter((snapshot) =>
      snapshot.snapshot?.data?.objectTypes.includes(
        ObjectTypes.Bookmark.toString(),
      ),
    )
    .map(fromAnytype);
}

function fromAnytype(snapshot: SnapshotWithType): ExternalBookmarkLink {
  const data = snapshot.snapshot?.data?.details;
  return {
    id: data?.id || "",
    _sid: getShortenId(snapshot),
    href: data?.name || "",
    title: data?.title || "",
    // level: data?.level || 0,
  };
}
