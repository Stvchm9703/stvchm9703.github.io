use crate::anytype_proto::anytype_model::{
    block::{Content, content},
    link_preview::Type as LinkPreviewType,
};

pub fn resolve_component_name(content: &Content) -> String {
    match content {
        Content::Smartblock(_) => "smart_block".to_owned(),
        Content::Text(t) => content::text::Style::from_i32(t.style)
            .unwrap()
            .as_str_name()
            .to_string(),
        Content::File(t) => content::file::Style::from_i32(t.style)
            .unwrap()
            .as_str_name()
            .to_string(),

        Content::Layout(t) => content::layout::Style::from_i32(t.style)
            .unwrap()
            .as_str_name()
            .to_string(),

        Content::Div(t) => {
            let divider = content::div::Style::from_i32(t.style)
                .unwrap()
                .as_str_name()
                .to_string();

            format!("divier_{}", divider)
        }

        // #[prost(message, tag = "18")]
        Content::Bookmark(t) => {
            let bookmark = LinkPreviewType::from_i32(t.r#type)
                .unwrap()
                .as_str_name()
                .to_string();
            format!("bookmark_{}", bookmark)
        }
        // #[prost(message, tag = "19")]
        Content::Icon(t) => "icon".to_string(),
        // #[prost(message, tag = "20")]
        Content::Link(t) => {
            let link = LinkPreviewType::from_i32(t.style)
                .unwrap()
                .as_str_name()
                .to_string();
            format!("link_{}", link)
        }
        // #[prost(message, tag = "21")]
        Content::Dataview(t) => format!("dataview_{}", t.active_view),
        // #[prost(message, tag = "22")]
        Content::Relation(_) => "relation".to_string(),
        // #[prost(message, tag = "23")]
        Content::FeaturedRelations(t) => "featured_relations".to_string(),
        // #[prost(message, tag = "24")]
        Content::Latex(t) => "latex".to_string(),
        // #[prost(message, tag = "25")]
        Content::TableOfContents(_) => "table_of_contents".to_string(),
        // #[prost(message, tag = "26")]
        Content::Table(_) => "table".to_owned(),
        // #[prost(message, tag = "27")]
        Content::TableColumn(_) => "table_column".to_owned(),
        // #[prost(message, tag = "28")]
        Content::TableRow(t) => {
            if t.is_header {
                "table_row_header".to_owned()
            } else {
                "table_row".to_owned()
            }
        }
        // #[prost(message, tag = "29")]
        Content::Widget(t) => format!("widget_{}", t.layout),
        // #[prost(message, tag = "30")]
        Content::Chat(t) => "chat".to_string(),

        _ => "".to_string(),
    }
}
