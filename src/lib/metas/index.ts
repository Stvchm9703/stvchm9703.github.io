import {
  BaseTwitterTags,
  BaseOGTags,
  BaseMetaTags,
  BaseAdditionalMetaTags,
  type IBaseAdditionalMetaTag,
} from "./base";

import type {
  MetaTagsProps,
  Twitter,
  OpenGraph,
  MetaTag,
  // deepMerge
} from "svelte-meta-tags";
import { deepMerge } from "svelte-meta-tags";

import { uniqBy } from "lodash-es";

export function resolveMetaTags(input: MetaTagsProps) {
  return {
    ...deepMerge(BaseMetaTags, input),
    twitter: {
      ...BaseTwitterTags,
      ...input.twitter,
    },
    openGraph: {
      ...BaseOGTags,
      ...input.openGraph,
    },
    additionalMetaTags: resolveAdditionalMetaTags(
      input.additionalMetaTags ?? [],
    ),
    additionalLinkTags: [
      ...BaseMetaTags.additionalLinkTags,
      ...(input.additionalLinkTags ?? []),
    ],
  } satisfies MetaTagsProps;
}

const resolveAdditionalMetaTags = (
  input: MetaTag[] | IBaseAdditionalMetaTag[],
) => {
  return uniqBy(
    [...input, ...BaseAdditionalMetaTags].map((tag) => ({
      name: tag.name ?? tag.property ?? tag.property,
      content: Array.isArray(tag.content)
        ? tag.content.join(", ")
        : tag.content,
    })),
    "name",
  );
};
