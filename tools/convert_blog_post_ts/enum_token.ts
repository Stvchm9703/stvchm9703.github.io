export enum ObjectTypes {
  // #[serde(rename )]
  Image = "ot-image",
  // #[serde(rename = "ot-participant")]
  Participant = "ot-participant",
  // #[serde(rename = "ot-page")]
  Page = "ot-page",
  // #[serde(rename = "ot-collection")]
  Collection = "ot-collection",
  // #[serde(rename = "ot-set")]
  Set = "ot-set",
  // #[serde(rename = "ot-bookmark")]
  Bookmark = "ot-bookmark",
  // #[serde(rename = "ot-space")]
  Space = "ot-space",
  // #[serde(rename = "ot-dashboard")]
  Dashboard = "ot-dashboard",
  // #[serde(rename = "ot-relation")]
  Relation = "ot-relation",
  // #[serde(rename = "ot-relationOption")]
  RelationOption = "ot-relationOption",
  // #[serde(rename = "ot-template")]
  Template = "ot-template",
  // #[serde(rename = "ot-task")]
  Task = "ot-task",
  // #[serde(rename = "ot-project")]
  Project = "ot-project",
  // #[serde(rename = "ot-objectType")]
  ObjectType = "ot-objectType",
  // #[serde(rename = "ot-file")]
  File = "ot-file",
  // #[serde(rename = "ot-audio")]
  Audio = "ot-audio",
  // #[serde(rename = "ot-note")]
  Note = "ot-note",
  // #[serde(rename = "ot-chatDerived")]
  ChatDerived = "ot-chatDerived",
  // #[serde(rename = "ot-video")]
  Video = "ot-video",
  // #[serde(rename = "ot-profile")]
  Profile = "ot-profile",
  // #[serde(rename = "ot-date")]
  Date = "ot-date",
  // #[serde(rename = "ot-spaceView")]
  SpaceView = "ot-spaceView",
}

export const getShortCode = (input: ObjectTypes): string => {
  switch (input) {
    case ObjectTypes.Relation:
      return "rl";
    case ObjectTypes.RelationOption:
      return "ro";
    case ObjectTypes.Template:
      return "tp";
    case ObjectTypes.Task:
      return "tk";
    case ObjectTypes.Project:
      return "pj";
    case ObjectTypes.ObjectType:
      return "ot";
    case ObjectTypes.File:
      return "fl";
    case ObjectTypes.Audio:
      return "ad";
    case ObjectTypes.Note:
      return "nt";
    case ObjectTypes.ChatDerived:
      return "cd";
    case ObjectTypes.Video:
      return "vd";
    case ObjectTypes.Profile:
      return "pf";
    case ObjectTypes.Date:
      return "dt";
    case ObjectTypes.SpaceView:
      return "sv";
  }
  return "ud";
};

// import { SmartBlockType } from "../../protos/anytype/models";

// console.log(SmartBlockType[SmartBlockType.Page]);
