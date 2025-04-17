export interface AnytypeFileObject {
  sbType: string;
  snapshot: Snapshot;
}

export interface Snapshot {
  logHeads: LogHeads;
  data: Data;
  fileKeys: any[];
}

export interface Data {
  blocks: Block[];
  details: Details;
  fileKeys: null;
  extraRelations: any[];
  objectTypes: string[];
  collections: null;
  removedCollectionKeys: any[];
  relationLinks: RelationLink[];
  key: string;
  originalCreatedTimestamp: string;
  fileInfo: null;
}

export interface Block {
  id: string;
  fields: Fields | null;
  restrictions: Restrictions | null;
  childrenIds: string[];
  backgroundColor: BackgroundColor;
  align: Align;
  verticalAlign: VerticalAlign;
  smartblock?: LogHeads;
  layout?: Div;
  text?: Text;
  featuredRelations?: LogHeads;
  relation?: Relation;
  tableOfContents?: LogHeads;
  bookmark?: Bookmark;
  div?: Div;
  table?: LogHeads;
  link?: Link;
  tableColumn?: LogHeads;
  tableRow?: TableRow;
}

export enum Align {
  AlignLeft = "AlignLeft",
}

export enum BackgroundColor {
  Empty = "",
  Lime = "lime",
  Orange = "orange",
  Red = "red",
}

export interface Bookmark {
  url: string;
  title: string;
  description: string;
  imageHash: string;
  faviconHash: string;
  type: string;
  targetObjectId: string;
  state: string;
}

export interface Div {
  style: string;
}

export interface LogHeads {}

export interface Fields {
  width?: number;
  _detailsKey?: string[];
}

export interface Link {
  targetBlockId: string;
  style: string;
  fields: null;
  iconSize: string;
  cardStyle: string;
  description: string;
  relations: any[];
}

export interface Relation {
  key: string;
}

export interface Restrictions {
  read: boolean;
  edit: boolean;
  remove: boolean;
  drag: boolean;
  dropOn: boolean;
}

export interface TableRow {
  isHeader: boolean;
}

export interface Text {
  text: string;
  style: Style;
  marks: Marks;
  checked: boolean;
  color: string;
  iconEmoji: string;
  iconImage: string;
}

export interface Marks {
  marks: Mark[];
}

export interface Mark {
  range: Range;
  type: Type;
  param: string;
}

export interface Range {
  from: number;
  to: number;
}

export enum Type {
  Bold = "Bold",
  Keyboard = "Keyboard",
  Link = "Link",
}

export enum Style {
  Header1 = "Header1",
  Header2 = "Header2",
  Header3 = "Header3",
  Marked = "Marked",
  Numbered = "Numbered",
  Paragraph = "Paragraph",
  Quote = "Quote",
  Title = "Title",
}

export enum VerticalAlign {
  VerticalAlignTop = "VerticalAlignTop",
}

export interface Details {
  backlinks: string[];
  createdDate: number;
  creator: string;
  description: string;
  featuredRelations: string[];
  id: string;
  lastModifiedBy: string;
  lastModifiedDate: number;
  lastOpenedDate: number;
  layout: number;
  links: string[];
  mentions: string[];
  name: string;
  snippet: string;
  sourceObject: string;
  spaceId: string;
  syncDate: number;
  syncError: number;
  syncStatus: number;
  tag: string[];
  type: string;
}

export interface RelationLink {
  key: string;
  format: string;
}
