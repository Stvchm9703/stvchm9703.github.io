import { SmartBlockType } from "../../protos/anytype/models";
import type { SnapshotWithType } from "../../protos/anytype/snapshot";
import type { AttributeMap } from "./attribute";
import type { CollectionMap } from "./collection";
import { getFieldValue } from "./common";
import type { PageMap } from "./page";
import type { Tag } from "./tag";
import type { User, UserMap } from "./user";

export type WorkspaceId = string;
export type WorkspaceMap = Map<WorkspaceId, Workspace>;

export interface Workspace {
  id: WorkspaceId;
  name: string;
  description: string;
  projects: PageMap;
  collections: CollectionMap;
  users: UserMap;
  tags: Map<string, Tag>;
  attributes: AttributeMap;
  // pub pages: HashMap<string, Page>,
  // pub relations: HashMap<string, Relation>,
}

export const getWorkspaces = (
  snapshot_list: SnapshotWithType[],
): Workspace[] => {
  return snapshot_list
    .filter((snapshot) => snapshot.sbType === SmartBlockType.Workspace)
    .map(fromAnytype);
};

export function fromAnytype(raw: SnapshotWithType): Workspace {
  const tmp: Workspace = {
    id: "",
    name: "",
    description: "",
    projects: new Map(),
    collections: new Map(),
    users: new Map(),
    tags: new Map(),
    attributes: new Map(),
  };
  const dataMap = raw.snapshot?.data?.details;

  if (!dataMap) {
    throw new Error("Data map is undefined");
  }
  // tmp.id = get_field_value(data_map, "id");
  tmp.id = getFieldValue(dataMap, "id");
  tmp.name = getFieldValue(dataMap, "name");
  // tmp.description = get_field_value(data_map, "description")?;
  // tmp.attributes = getFieldValue(dataMap, "attributes");
  // dataMap.get("attributes")?.value?.map_value?.fields ?? new Map();

  return tmp;
}

export function addUser(self: Workspace, user: User) {
  // self.users.insert(user.id.clone(), user.clone());
  self.users.set(user.id, user);
}
