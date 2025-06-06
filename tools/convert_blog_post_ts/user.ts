import type { SnapshotWithType } from "../../protos/anytype/snapshot";
import { castToAttributeMap, type AttributeMap } from "./attribute";
import { getFieldValue, getShortenId } from "./common";

export type UserId = string;
export type UserMap = Map<UserId, User>;

export interface User {
  id: UserId;
  _sid: string;
  name: string;
  email: string;
  description: string;
  attributes: AttributeMap;
}

export function fromAnytype(raw: SnapshotWithType): User {
  const tmp: User = {
    id: "",
    _sid: "",
    name: "",
    email: "",
    description: "",
    attributes: {} as AttributeMap,
  };

  const dataMap = raw.snapshot?.data?.details;
  if (!dataMap) {
    throw new Error("Data map is undefined");
  }

  tmp.id = getFieldValue(dataMap, "id");
  tmp._sid = getShortenId(raw);
  tmp.name = getFieldValue(dataMap, "name");
  tmp.description = getFieldValue(dataMap, "description");
  tmp.attributes = castToAttributeMap(dataMap);

  return tmp;
}
