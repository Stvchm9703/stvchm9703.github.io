use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SbType {
    Page,
    FileObject,
    Participant,
    Workspace,
    Widget,
    STRelation,
    STRelationOption,
    Template,
    STType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardStyle {
    Card,
    Text,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FormulaEnum {
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IconSize {
    #[serde(rename = "SizeNone")]
    None,
    #[serde(rename = "SizeSmall")]
    Small,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkType {
    Bold,
    Keyboard,
    Link,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextStyle {
    Description,
    Header1,
    Header2,
    Header3,
    Marked,
    Numbered,
    Paragraph,
    Quote,
    Title,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerticalAlign {
    #[serde(rename = "VerticalAlignTop")]
    Top,
    #[serde(rename = "VerticalAlignMiddle")]
    Middle,
    #[serde(rename = "VerticalAlignBottom")]
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Checkbox,
    Date,
    Emoji,
    File,
    Longtext,
    Number,
    Object,
    Shorttext,
    Status,
    Tag,
    Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DateFormat {
    #[serde(rename = "MonthAbbrBeforeDay")]
    MonthAbbrBeforeDay,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeFormat {
    Format12,
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum BackgroundColor {
//     #[serde(rename = "")]
//     Empty,
//     Lime,
//     Orange,
//     Red,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Align {
    #[serde(rename = "AlignLeft")]
    Left,

    #[serde(rename = "AlignCenter")]
    Center,

    #[serde(rename = "AlightRight")]
    Right,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObjectType {
    #[serde(rename = "ot-image")]
    Image,
    #[serde(rename = "ot-participant")]
    Participant,
    #[serde(rename = "ot-page")]
    Page,
    #[serde(rename = "ot-collection")]
    Collection,
    #[serde(rename = "ot-set")]
    Set,
    #[serde(rename = "ot-bookmark")]
    Bookmark,
    #[serde(rename = "ot-space")]
    Space,
    #[serde(rename = "ot-dashboard")]
    Dashboard,
    #[serde(rename = "ot-relation")]
    Relation,
    #[serde(rename = "ot-relationOption")]
    RelationOption,
    #[serde(rename = "ot-template")]
    Template,
    #[serde(rename = "ot-task")]
    Task,
    #[serde(rename = "ot-project")]
    Project,
    #[serde(rename = "ot-objectType")]
    ObjectType,
    #[serde(rename = "ot-file")]
    File,
    #[serde(rename = "ot-audio")]
    Audio,
    #[serde(rename = "ot-note")]
    Note,
    #[serde(rename = "ot-chatDerived")]
    ChatDerived,
    #[serde(rename = "ot-video")]
    Video,
    #[serde(rename = "ot-profile")]
    Profile,
    #[serde(rename = "ot-date")]
    Date,
    #[serde(rename = "ot-spaceView")]
    SpaceView,
}

impl ToString for ObjectType {
    fn to_string(&self) -> String {
        match self {
            ObjectType::Image => "ot-image".to_string(),
            ObjectType::Participant => "ot-participant".to_string(),
            ObjectType::Page => "ot-page".to_string(),
            ObjectType::Collection => "ot-collection".to_string(),
            ObjectType::Set => "ot-set".to_string(),
            ObjectType::Bookmark => "ot-bookmark".to_string(),
            ObjectType::Space => "ot-space".to_string(),
            ObjectType::Dashboard => "ot-dashboard".to_string(),
            ObjectType::Relation => "ot-relation".to_string(),
            ObjectType::RelationOption => "ot-relationOption".to_string(),
            ObjectType::Template => "ot-template".to_string(),
            ObjectType::Task => "ot-task".to_string(),
            ObjectType::Project => "ot-project".to_string(),
            ObjectType::ObjectType => "ot-objectType".to_string(),
            ObjectType::File => "ot-file".to_string(),
            ObjectType::Audio => "ot-audio".to_string(),
            ObjectType::Note => "ot-note".to_string(),
            ObjectType::ChatDerived => "ot-chatDerived".to_string(),
            ObjectType::Video => "ot-video".to_string(),
            ObjectType::Profile => "ot-profile".to_string(),
            ObjectType::Date => "ot-date".to_string(),
            ObjectType::SpaceView => "ot-spaceView".to_string(),
        }
    }
}
