import { SnapshotWithType } from "../../protos/anytype/snapshot";
import { ObjectTypes } from "./enum_token";

export interface ExternalBookmarkLink {
  id: string;
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
    href: data?.name || "",
    title: data?.title || "",
    // level: data?.level || 0,
  };
}
