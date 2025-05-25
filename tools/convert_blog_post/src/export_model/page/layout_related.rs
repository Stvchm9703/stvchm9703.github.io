use anyhow::{Error, anyhow};
use smallvec::{SmallVec, smallvec};

use super::Page;
use crate::export_model::content_block::{
    ComponentAttrType, ContentBlock,
    layout::{LayoutComponentAttr, LayoutItem},
    text::{TextComponentAttr, TextItem, TextStyle},
};

#[derive(Debug, Default)]
struct PageTextListSet {
    pub style: TextStyle,
    pub start: usize,
    pub end: usize,
    pub is_header: bool,
    pub start_id: String,
    pub sebering_id_list: Vec<String>,
    pub start_child: Vec<String>,
    pub layer: usize,
}

impl PageTextListSet {
    pub fn is_sebering(&self, style: &TextStyle, order: &usize, layer: &usize) -> bool {
        return &self.style == style && order > &self.end && &self.layer == layer;
    }

    pub fn push_next(&mut self, blk: &ContentBlock) -> Result<(), Error> {
        if self.end + 1 > blk.order {
            return Err(anyhow!("unresovled"));
        }

        self.end = blk.order.to_owned();
        self.sebering_id_list.push(blk.id.to_owned());
        // self.4 = layer;
        //
        return Ok(());
    }
}

type PageTextListMainSet = SmallVec<[PageTextListSet; 128]>;
impl Page {
    // #[deprecated]
    pub(super) fn resolve_children_ids_order(
        &mut self,
        check_id: &str,
        order_list_ptr: &mut Vec<String>,
    ) {
        let root_block = self.cache_contents.get(check_id);
        if root_block.is_none() {
            return;
        }

        let root_blk = root_block.unwrap().to_owned();

        if root_blk.id == "header" || root_blk.id == "title" || root_blk.id == "featuredrelations" {
            return;
        }

        // layer check and filter children
        let mut layout_comp_is_div: bool = false;
        let mut layout_comp_is_layered: bool = false;
        // let mut is_layout_comp: bool = false;
        if let ComponentAttrType::Layout(l) = &root_blk.component_attr {
            // is_layout_comp = true;
            layout_comp_is_div = l.layout_style == "Div";
            layout_comp_is_layered = l.layout_style != "Div";
        }
        let mut text_comp_is_toggle: bool = false;
        let mut text_comp_is_layer_child: bool = false;
        if let ComponentAttrType::Text(t) = &root_blk.component_attr {
            text_comp_is_toggle = t.style == TextStyle::Toggle;
            if t.style == TextStyle::Marked || t.style == TextStyle::Numbered {
                text_comp_is_layer_child =
                    root_blk.children_ids.as_ref().is_some_and(|f| f.len() > 0);
            }
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
            || text_comp_is_toggle
            || text_comp_is_layer_child
        {
            return;
        }

        let chil_ids = root_blk.children_ids.to_owned().unwrap_or_default();
        for elm in chil_ids {
            self.resolve_children_ids_order(&elm, order_list_ptr);
        }

        if (root_blk.debug_type == "layout" && layout_comp_is_div) || root_blk.debug_type == "div" {
            //     // order_list.push(check_id.to_owned());
            order_list_ptr.push(check_id.to_string());
        }

        // drop(root_blk);

        return;
    }

    // type TmpTextListSizer = ;

    pub(super) fn resolve_nest_children(&mut self, order_list: &Vec<String>) {
        let mut tmp_cache_contents = self.cache_contents.to_owned();
        let mut tmp_text_mark_and_number: PageTextListMainSet = smallvec![];

        if let Some(root_blk) = tmp_cache_contents.get(&self.id) {
            // add default layer
            tmp_text_mark_and_number.push(PageTextListSet {
                style: TextStyle::Title,
                is_header: true,
                start_id: root_blk.id.to_owned(),
                start_child: root_blk.children_ids.to_owned().unwrap_or_default(),
                ..PageTextListSet::default()
            });
        }

        let current_layer = 1;

        for id in order_list {
            let ct_blk = tmp_cache_contents.get_mut(id);

            if let Some(blk) = ct_blk {
                self.resolve_internal_children_pipeline(
                    blk,
                    &mut tmp_text_mark_and_number,
                    current_layer,
                );
                self.contents.push(blk.to_owned());
            }
        }

        if let Some(root_blk) = tmp_cache_contents.get_mut(&self.id) {
            // self.transform_text_list(root_blk, &mut tmp_text_mark_and_number, 0);
            let child = tmp_text_mark_and_number
                .iter()
                .filter(|p| {
                    p.layer == 1
                        && root_blk
                            .children_ids
                            .as_ref()
                            .is_some_and(|f| f.contains(&p.start_id))
                })
                .collect::<Vec<_>>();

            let mut itemv: Vec<TextItem> = vec![];
            let mut subitemvec: Vec<TextItem> = vec![];

            for &item in child.iter() {
                if let Some(s) = tmp_cache_contents.get(&item.start_id) {
                    let subitem = TextItem::to_subitem(s);
                    if matches!(subitem, TextItem::LevelText(_)) {
                        if !subitemvec.is_empty() {
                            itemv.push(TextItem::create_container(&subitemvec));
                        }
                        subitemvec = vec![subitem];
                    } else {
                        subitemvec.push(subitem);
                    }
                }
            }

            if !subitemvec.is_empty() {
                itemv.push(TextItem::create_container(&subitemvec));
            }

            // println!("itemc : {:}")

            // if self.id == "bafyreialni2kwfzmrbytsbg3xl4zwug4mp4vbxxri5tcpdddtrfraexhha" {
            //     println!("itemc : {:#?}", itemv);

            //     println!("text_sebering - snapshot: {:#?}", child);
            //     // println!("curr-lyr : {:?}", current_layer);
            //     // println!("child-lyr : {:#?}", child_layer);
            // }
        }
    }

    fn resolve_internal_children_pipeline(
        &mut self,
        current_block: &mut ContentBlock,
        text_sebering: &mut PageTextListMainSet,
        current_layer: usize,
    ) {
        // if self.id == "bafyreialni2kwfzmrbytsbg3xl4zwug4mp4vbxxri5tcpdddtrfraexhha" {
        //     println!("current_block : {:#?}", current_block);
        // }
        self.resolve_layout_nest_children(current_block, text_sebering, current_layer);
        self.resolve_text_nest_children(current_block, text_sebering, current_layer);
    }

    fn resolve_text_nest_children(
        &mut self,
        current_block: &mut ContentBlock,
        text_sebering: &mut PageTextListMainSet,
        current_layer: usize,
    ) {
        let attr: &mut TextComponentAttr;
        let cloned_curr_blk = current_block.to_owned();

        match &mut current_block.component_attr {
            ComponentAttrType::Text(e) => attr = e,
            _ => return,
        }
        if [TextStyle::Marked, TextStyle::Numbered, TextStyle::Toggle].contains(&attr.style)
            == false
        {
            return;
        }
        // println!(in );

        let lk_children_ids = current_block.children_ids.as_ref().unwrap();
        let mut cached = self.cache_contents.to_owned();
        // println!("attr : {:#?}", attr);

        // if attr.style == TextStyle::Toggle {
        for child_id in lk_children_ids.iter() {
            if let Some(child_blk) = cached.get_mut(child_id) {
                if child_blk.children_ids.is_some() {
                    self.resolve_internal_children_pipeline(
                        child_blk,
                        text_sebering,
                        current_layer + 1,
                    );
                }

                if attr.style == TextStyle::Toggle {
                    // attr.items
                    let item_converted = TextItem::to_subitem(child_blk);
                    if attr.items.is_none() {
                        attr.items = Some(vec![]);
                    }
                    if let Some(y) = attr.items.as_mut() {
                        y.push(item_converted);
                    }
                }
            }
        }

        if attr.style == TextStyle::Numbered || attr.style == TextStyle::Marked {
            let sebering = text_sebering
                .iter_mut()
                .find(|p| p.is_sebering(&attr.style, &cloned_curr_blk.order, &current_layer));
            if let Some(last_item) = sebering {
                // find next?
                let _ = last_item.push_next(&cloned_curr_blk);
            } else {
                text_sebering.push(PageTextListSet {
                    style: attr.style,
                    is_header: false,
                    start: cloned_curr_blk.order,
                    start_id: cloned_curr_blk.id.to_owned(),
                    start_child: lk_children_ids.to_vec(),
                    end: cloned_curr_blk.order,
                    sebering_id_list: vec![cloned_curr_blk.id.to_owned()],
                    layer: current_layer,
                });
            }
        }

        self.transform_text_list(current_block, text_sebering, current_layer);
    }

    fn transform_text_list(
        &mut self,
        current_block: &mut ContentBlock,
        text_sebering: &mut PageTextListMainSet,
        current_layer: usize,
    ) {
        if text_sebering.len() == 0 {
            return;
        }
        let cached = self.cache_contents.to_owned();

        let child_layer = text_sebering
            .iter()
            .filter(|p| {
                p.layer == current_layer + 1
                    && current_block
                        .children_ids
                        .as_ref()
                        .is_some_and(|f| f.contains(&p.start_id))
            })
            .collect::<Vec<_>>();

        // if self.id == "bafyreialni2kwfzmrbytsbg3xl4zwug4mp4vbxxri5tcpdddtrfraexhha" {
        //     println!("text_sebering - snapshot: {:#?}", text_sebering);
        //     println!("curr-lyr : {:?}", current_layer);
        //     println!("child-lyr : {:#?}", child_layer);
        // }
        if child_layer.len() > 0 {
            let mut itemv: Vec<TextItem> = vec![];
            let mut subitemvec: Vec<TextItem> = vec![];
            // let mut is_
            match &mut current_block.component_attr {
                ComponentAttrType::Text(text) => {
                    if let Some(r) = current_block.children_ids.to_owned() {
                        for it in r.iter() {
                            if let Some(s) = cached.get(it) {
                                let subitem = TextItem::to_subitem(s);
                                if matches!(subitem, TextItem::LevelText(_)) {
                                    if !subitemvec.is_empty() {
                                        itemv.push(TextItem::create_container(&subitemvec));
                                    }
                                    subitemvec = vec![subitem];
                                } else {
                                    subitemvec.push(subitem);
                                }
                            }
                        }
                    }
                    if !subitemvec.is_empty() {
                        itemv.push(TextItem::create_container(&subitemvec));
                    }
                    text.items = Some(itemv);
                    // current_block.component_attr = ComponentAttrType::Text(text.to_owned());
                    // println!("cur-blk : {:#?}", current_block);
                }
                _ => {}
            }
        }

        let new_block = current_block.clone();
        self.cache_contents.remove(&current_block.id);
        let _ = self
            .cache_contents
            .insert(new_block.id.to_owned(), new_block.to_owned());
        // text_sebering.clear();
    }
    fn resolve_layout_nest_children(
        &mut self,
        current_block: &mut ContentBlock,
        text_sebering: &mut PageTextListMainSet,
        current_layer: usize,
    ) {
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
                if child_blk.children_ids.is_some() {
                    self.resolve_internal_children_pipeline(
                        child_blk,
                        text_sebering,
                        current_layer + 1,
                    );
                }

                let layout_item = LayoutItem::from_content_block(child_blk);

                if let Err(e) = attr.push_item(layout_item) {
                    println!("error throw in {:?}", e);
                }
            }
        }
    }
}
