// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export type AttributeMap = Map<string, any>;

export const castToAttributeMap = (dataMap?: {
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  [key: string]: any;
}): AttributeMap => {
  if (!dataMap) {
    return new Map();
  }
  return new Map(Object.entries(dataMap));
};
