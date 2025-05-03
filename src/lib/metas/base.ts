import type { MetaTagsProps, Twitter, OpenGraph } from "svelte-meta-tags";
import { BASE } from "$env/static/private";
export const BaseTwitterTags: Twitter = {
  cardType: "summary_large_image",
  site: "@steven9703_github_io",
  creator: "@hassan_ao_ddid",
  title: "Steven Dev's Portfolio",
  description: "Steven Dev's Portfolio",
  image: `${BASE}/images/meta_cover.jpg`,
};

export const BaseOGTags: OpenGraph = {
  type: "website",
  title: "Steven Dev's Portfolio",
  url: BASE,
  description: "Steven Dev's Portfolio",
  siteName: "Steven Dev's Portfolio",
  // image: "https://steven.dev/images/steven-dev.png",
  images: [
    {
      url: `${BASE}/images/meta_cover.jpg`,
      alt: "Steven Dev's Portfolio",
      width: 800,
      height: 600,
    },
  ],
};

export interface IBaseAdditionalMetaTag {
  name: string;
  content: string | string[];
}

export const BaseAdditionalMetaTags: IBaseAdditionalMetaTag[] = [
  {
    name: "keywords",
    content: ["Steven Dev", "Portfolio", "Developer", "Software Engineer"],
  },
];

export const BaseMetaTags: MetaTagsProps = {
  title: "Steven Dev's Portfolio",
  titleTemplate: "%s | Steven Dev's Portfolio",
  robots: "index, follow",
  description: "Steven Dev's Portfolio",
  canonical: BASE,
  // twitter: BaseTwitterTags,
  // openGraph: BaseOGTags,
  // additionalMetaTags: AdditionalMetaTags,
  additionalLinkTags: [
    {
      rel: "icon",
      href: "/favicon.svg",
    },
  ],
};
