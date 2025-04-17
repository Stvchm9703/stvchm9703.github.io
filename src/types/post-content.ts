export interface PostContent {
  title: string;
  date: string;
  readTime: string;
  author: string;
  authorRole: string;
  $schema: string;
  content: Content[];
  tags: string[];
  relatedPosts: RelatedPost[];
  relatedChapters: RelatedChapter[];
  tableOfContents?: TableOfContent[];
}

export type Content =
  | ParagraphContent
  | ImageContent
  | CodeContent
  | QuoteContent
  | VideoContent
  | AudioContent
  | TableContent
  | HeadingContent
  | EmbedContent;

export interface BaseContentAttr {
  block_id?: string;
  className?: string;
  type: string;
}

export interface ParagraphContent extends BaseContentAttr {
  type: "paragraph";
  content?: string;
}

export interface ImageContent extends BaseContentAttr {
  type: "image";
  src?: string;
  alt?: string;
  caption?: string;
}

export interface CodeContent extends BaseContentAttr {
  type: "code";
  language?: string;
  content?: string;
}

export interface QuoteContent extends BaseContentAttr {
  type: "quote";
  content?: string;
  author?: string;
}

export interface VideoContent extends BaseContentAttr {
  type: "video";
  src?: string;
  caption?: string;
}

export interface AudioContent extends BaseContentAttr {
  type: "audio";
  src?: string;
  caption?: string;
}

export interface TableContent extends BaseContentAttr {
  type: "datatable";
  content?: string;
}

export interface HeadingContent extends BaseContentAttr {
  id: string;
  type: "heading";
  content?: string;
}

export interface EmbedContent extends BaseContentAttr {
  type: "embed";
  content?: string;
}

export interface RelatedPost {
  id: number;
  title: string;
}

export interface RelatedChapter {
  id: number;
  title: string;
  date: string;
}

export interface TableOfContent {
  id: string;
  title: string;
}
