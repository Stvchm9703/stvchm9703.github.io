use super::{
    common::{
        AttributeMap, DEFAULT_TAG, get_field_value, get_shorten_id, get_snapshot_shorthanded,
        is_release, path_resolver,
    },
    content_block::{
        ComponentAttrType, ContentBlock,
        layout::{LayoutComponentAttr, LayoutItem},
        text::TextStyle,
    },
    external_link, file_object,
    page_attr::*,
    page_ext::*,
    trait_impl::{FromBlock, FromRaw, FromSnapshotList},
};
use crate::{
    export_model::common::{GLOBAL_RELATION_NAMEMAP, GLOBAL_RELATION_NAMEMAP_STR},
    proto::anytype::SnapshotWithType,
};

use std::{borrow::BorrowMut, collections::BTreeMap};

use anyhow::anyhow;
use convert_blog_post_marco::set_field_value;
use serde::{Deserialize, Serialize};

use lo_::intersection;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    #[serde(rename = "_sid")]
    pub sid: String,
    pub title: String,
    // pub description: String,
    pub snippet: String,
    pub collection_id: String,
    pub workspace_id: String,
    // pub attributes: PageAttributes,
    #[serde(skip_serializing_if = "is_release")]
    pub raw_attributes: AttributeMap,
    pub creator: String,
    pub publish_date: i64,
    pub tags: Vec<PageExternalLink>,
    #[serde(skip_serializing_if = "is_release")]
    pub raw_tag_list: Vec<String>,
    pub styles: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "is_release")]
    pub raw_series: Option<Vec<String>>,
    pub serie: Option<PageExternalLink>,
    // page other relation / attr
    pub meta: PageMeta,
    pub table_of_contents: Vec<PageExternalLink>,
    pub related_chapters: Vec<PageExternalLink>,
    pub related_articles: Vec<PageExternalLink>,
    // content-block
    #[serde(skip)]
    pub cache_contents: BTreeMap<String, ContentBlock>,
    #[serde(skip_serializing_if = "is_release")]
    pub check_block_order: Vec<String>,

    pub contents: Vec<ContentBlock>,
}

impl<'a> FromRaw<SnapshotWithType<'a>, Page> for Page {
    fn from_raw(input: &SnapshotWithType) -> Result<Page, anyhow::Error> {
        let mut tmp = Page::default();
        let instance = get_snapshot_shorthanded(input);
        if let Err(e) = instance {
            return Err(anyhow!("fail field map : {:?}", e));
        }
        let (data_wrap, field_map) = instance.unwrap();
        tmp.raw_attributes = field_map.clone();
        // let field_map_ref = field_map;
        let field_try_map = get_page_details(&data_wrap);
        if let Err(e) = field_try_map {
            // eprintln!("{:?}", e);
            return Err(anyhow!("fail page-attributes : {:?}", e));
        }
        let field_attr = field_try_map.unwrap();

        // if let Ok(id) = get_field_value::<String>(field_map_ref, "id") {
        tmp.id = field_attr.id.to_owned();
        tmp.sid = get_shorten_id(&field_attr.id);
        // };
        // if let Ok(title) = get_field_value::<String>(field_map_ref, "name") {
        tmp.title = field_attr.name.to_owned();
        // }
        // tmp.attributes = field_attr.to_owned();
        // if let Ok(backlink) =  get_field_value::<Vec<String>>>(field_map_ref, "backlinks"){
        if field_attr.backlinks.len() > 0 {
            tmp.collection_id = field_attr.backlinks.into_iter().last().unwrap().to_owned();
        }

        // tmp.workspace_id = getFieldValue(detailMap, "spaceId") ?? "";
        tmp.workspace_id = field_attr.space_id;

        // tmp.creator = getFieldValue(detailMap, "creator") ?? "";
        tmp.creator = field_attr.creator;
        // tmp.snippet = getFieldValue(detailMap, "snippet") ?? "";
        tmp.snippet = field_attr.snippet;
        // tmp.raw_tag_list = getFieldValue(detailMap, "tag") ?? [];
        if let Some(tag) = field_attr.tag {
            tmp.raw_tag_list = tag;
        }
        // tmp.publish_date = getFieldValue<number>(detailMap, "publish date");
        if let Ok(publish_date) = get_field_value::<i64>(&field_map, "publish date") {
            tmp.publish_date = publish_date.to_owned();
        }

        for (idx, block) in data_wrap.blocks.iter().enumerate() {
            let block_tmp = ContentBlock::from_block_with_idx(block, idx);
            if let Ok(blk) = block_tmp {
                tmp.cache_contents.insert(blk.id.to_owned(), blk.clone());
            } else {
                println!("error in page-> content-block : {:?}", block_tmp.err());
            }
        }

        return Ok(tmp);
    }
}

impl FromSnapshotList<Page> for Page {
    fn from_snapshot_list(raw: Vec<SnapshotWithType>) -> Result<Vec<Page>, anyhow::Error> {
        let mut tmp_ls: Vec<Page> = vec![];

        for y in raw.into_iter() {
            // return Page::from_raw(&raw).unwrap();
            let tmp = Page::from_raw(&y);
            if let Err(e) = tmp {
                return Err(anyhow!("list init error : {:?} , {:?}", e, &y));
            }
            tmp_ls.push(tmp.unwrap());
        }

        return Ok(tmp_ls);
    }
}

impl ToPageExternalLink for Page {
    fn to_page_ext_link(&self) -> PageExternalLink {
        return PageExternalLink {
            id: self.id.to_string(),
            sid: get_shorten_id(self.id.as_str()),
            label: self.title.to_string(),
            url: format!(
                "/posts/{}_{}",
                get_shorten_id(self.id.as_str()),
                path_resolver(self.title.as_str())
            ),
            ..Default::default()
        };
    }
    fn to_page_external_link(&self) -> PageExternalLink {
        self.to_page_ext_link()
    }
}
impl Page {
    pub fn recheck_fields(&mut self) {
        let mut publish_date: f64 = 0.0;
        set_field_value!(publish_date, self.raw_attributes, "publish date");
        self.publish_date = publish_date as i64;
        let mut series: Vec<String> = Vec::<String>::new();
        set_field_value!(series, self.raw_attributes, "Series");
        self.raw_series = Some(series.to_owned());
        if let Some(rel_key) = GLOBAL_RELATION_NAMEMAP_STR.lock().unwrap().get("Series") {
            if let Some(serie_id) = series.first() {
                if let Some(tag) = GLOBAL_RELATION_NAMEMAP.lock().unwrap().get(rel_key) {
                    self.serie = tag.get_option_page_link(serie_id);
                }
            }
        }

        // GLOBAL_RELATION_NAMEMAP.

        // println!("GLOBAL_RELATION_IDMAP {:?}", GLOBAL_RELATION_IDMAP);
        // println!("GLOBAL_RELATION_NAMEMAP {:?}", GLOBAL_RELATION_NAMEMAP);
        // println!( "GLOBAL_RELATION_NAMEMAP_STR {:?}", GLOBAL_RELATION_NAMEMAP_STR );
    }

    pub fn resolve_content_block(&mut self) {
        let root_block = self.cache_contents.get(&self.id);
        if root_block.is_none() {
            return;
        }
        let check_id = self.id.to_owned();
        let mut order_list_ptr = vec![];
        self.resolve_children_ids_order(&check_id, &mut order_list_ptr);
        self.resolve_nest_children(&order_list_ptr);
        return;
    }
    // #[deprecated]
    fn resolve_children_ids_order(&mut self, check_id: &str, order_list_ptr: &mut Vec<String>) {
        let root_block = self.cache_contents.get(check_id);
        if root_block.is_none() {
            return;
        }

        let root_blk = root_block.unwrap().to_owned();

        if root_blk.id == "header" || root_blk.id == "title" || root_blk.id == "featuredrelations" {
            return;
        }

        let mut layout_comp_is_div: bool = false;
        let mut layout_comp_is_layered: bool = false;
        if let ComponentAttrType::Layout(l) = root_blk.component_attr {
            layout_comp_is_div = (l.layout_style == "Div");
            layout_comp_is_layered = (l.layout_style != "Div");
        }
        // let default_skip_component

        if root_blk.debug_type != "div"
            && root_blk.debug_type != "relation"
            && root_blk.debug_type != "table_of_contents"
            && root_blk.debug_type != "smartblock"
            && (root_blk.debug_type == "layout" && layout_comp_is_div == true) == false
        {
            order_list_ptr.push(check_id.to_string());
        }

        // skip the children
        if root_blk.debug_type == "table"
            || (root_blk.debug_type == "layout" && layout_comp_is_layered)
        {
            return;
        }

        let chil_ids = root_blk.children_ids.to_owned().unwrap_or_default();
        for elm in chil_ids {
            self.resolve_children_ids_order(&elm, order_list_ptr);
        }

        // if (root_blk.debug_type == "layout" && layout_comp_is_div) || root_blk.debug_type == "div" {
        //     // order_list.push(check_id.to_owned());
        //     order_list_ptr.push(check_id.to_string());
        // }

        // drop(root_blk);

        return;
    }

    fn resolve_nest_children(&mut self, order_list: &Vec<String>) {
        let mut tmp_cache_contents = self.cache_contents.to_owned();
        for id in order_list {
            let ct_blk = tmp_cache_contents.get_mut(id);

            if let Some(blk) = ct_blk {
                self.resolve_layout_nest_children(blk);
                self.contents.push(blk.to_owned());
            }
        }
    }

    fn resolve_layout_nest_children(&mut self, current_block: &mut ContentBlock) {
        let attr: &mut LayoutComponentAttr;
        match &mut current_block.component_attr {
            ComponentAttrType::Layout(e) | ComponentAttrType::Table(e) => attr = e,
            _ => {
                // println!("skip resolve_layout_nest_children");
                return;
            }
        }

        let lk_children_ids = current_block.children_ids.as_ref().unwrap();
        let mut cached = self.cache_contents.to_owned();

        for child_id in lk_children_ids.into_iter() {
            let child_blk_result = cached.get_mut(child_id);
            if let Some(child_blk) = child_blk_result {
                if let Some(_) = child_blk.children_ids.as_ref() {
                    self.resolve_layout_nest_children(child_blk);
                }

                let layout_item = LayoutItem::from_content_block(child_blk);

                if let Err(e) = attr.push_item(layout_item) {
                    println!("error throw in {:?}", e);
                }
            }
        }
    }

    pub(crate) fn resolve_external_link(
        &mut self,
        external_link: &Vec<external_link::ExternalBookmarkLink>,
    ) {
        let list = self.cache_contents.values_mut();
        for item in list {
            if let ComponentAttrType::Link(inside) = item.component_attr.borrow_mut() {
                let target_link = external_link
                    .iter()
                    .find(|p| p.id == inside.target_block_id);
                if let Some(tar) = target_link {
                    if tar.title.is_empty() == false {
                        inside.title = tar.title.to_string();
                    }
                    inside.url = tar.url.to_string();
                }
            }
        }
    }

    pub(crate) fn resolve_file_link(&mut self, file_link: &Vec<file_object::FileObject>) {
        let list = self.cache_contents.values_mut();
        let mut file_list = vec![];
        for item in list {
            if let ComponentAttrType::File(inside) = item.component_attr.borrow_mut() {
                let target_link = file_link.iter().find(|p| p.id == inside.target_object_id);
                if let Some(tar) = target_link {
                    inside.file_url = tar.file_url.to_string();
                    file_list.push(tar);
                }
            }
        }
        self.add_meta_data(&file_list);
    }

    fn add_meta_data(&mut self, file_list: &Vec<&file_object::FileObject>) {
        let images = file_list
            .into_iter()
            .filter(|p| p.file_type == "images")
            .map(|p| p.to_page_meta_og())
            .collect::<Vec<PageMetaOpenGraphObj>>();

        let videos = file_list
            .into_iter()
            .filter(|p| p.file_type == "videos")
            .map(|p| p.to_page_meta_og())
            .collect::<Vec<PageMetaOpenGraphObj>>();

        let audio = file_list
            .into_iter()
            .filter(|p| p.file_type == "audios")
            .map(|p| p.to_page_meta_og())
            .collect::<Vec<PageMetaOpenGraphObj>>();

        if images.len() > 0 {
            self.meta.images = Some(images);
        }
        if videos.len() > 0 {
            self.meta.videos = Some(videos);
        }
        if audio.len() > 0 {
            self.meta.audio = Some(audio);
        }
    }

    pub(crate) fn resolve_toc_component(&mut self) {
        let toc_list = self
            .cache_contents
            .values()
            .into_iter()
            .filter(|p| {
                if let ComponentAttrType::Text(attr) = &p.component_attr {
                    return attr.style == TextStyle::Header1
                        || attr.style == TextStyle::Header2
                        || attr.style == TextStyle::Header3
                        || attr.style == TextStyle::Header4;
                }
                // p.debug_type == "text"
                return false;
            })
            .map(|p| p.to_toc_link().unwrap())
            .collect::<Vec<_>>();
        self.table_of_contents = toc_list;
    }

    pub(crate) fn resolve_tag_link(&mut self) {
        // println!("tag-list :{:?}", tag_list);
        // self.raw_tag_list.iter().map(f)
        let included_tag = DEFAULT_TAG
            .lock()
            .unwrap()
            .options
            .iter()
            .filter(|p| self.raw_tag_list.contains(&p.id))
            .map(|f| f.to_page_ext_link())
            .collect::<Vec<_>>();

        self.tags = included_tag;
    }

    pub(crate) fn resolve_related_chapters(&mut self, page_list: &Vec<Page>) {
        if self.serie.is_none() {
            return;
        }
        let serie_tag = self.serie.as_ref().unwrap();
        let mut d = page_list
            .iter()
            .filter(|p| {
                if p.serie.is_none() {
                    return false;
                }
                return p.serie.as_ref().unwrap().id == serie_tag.id;
            })
            .collect::<Vec<_>>();
        d.sort_by(|a, b| b.publish_date.cmp(&a.publish_date));

        self.related_chapters = d
            .iter()
            .take(5)
            .map(|p| p.to_page_ext_link())
            .collect::<Vec<_>>();
    }

    pub(crate) fn resolve_related_articles(&mut self, page_list: &Vec<Page>) {
        if self.raw_tag_list.len() == 0 {
            return;
        }
        let mut d = page_list
            .iter()
            .filter(|p| {
                let ttt = intersection(&self.raw_tag_list, &p.raw_tag_list);
                return ttt.len() > 0;
            })
            .collect::<Vec<_>>();
        d.sort_by(|a, b| b.publish_date.cmp(&a.publish_date));

        self.related_articles = d
            .iter()
            .take(5)
            .map(|p| p.to_page_ext_link())
            .collect::<Vec<_>>();
    }

    pub(crate) fn to_post_card_link(&self) -> PageExternalLink {
        let mut tmp = self.to_page_ext_link();
        tmp.snippet = Some(self.snippet.to_string());
        tmp.tags = Some(self.tags.to_owned());

        return tmp;
    }
}
