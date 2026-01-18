use anyhow::{Error, anyhow};
use smallvec::{SmallVec, smallvec};

use super::Page;
use crate::export_model::content_block::{
    ComponentAttrType, ContentBlock,
    layout::{LayoutComponentAttr, LayoutItem},
    text::{TextComponentAttr, TextItem, TextStyle},
};

#[derive(Debug, Default, Clone)]
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
    /// Check if a block can be added to this sibering group.
    /// Must be same style, same layer, and consecutive order (end + 1 == new order)
    pub fn can_continue_sibering(&self, style: &TextStyle, order: &usize, layer: &usize) -> bool {
        &self.style == style && &self.layer == layer && self.end + 1 == *order
    }

    pub fn push_next(&mut self, blk: &ContentBlock) -> Result<(), Error> {
        if self.end + 1 != blk.order {
            return Err(anyhow!(
                "not consecutive: expected order {}, got {}",
                self.end + 1,
                blk.order
            ));
        }

        self.end = blk.order;
        self.sebering_id_list.push(blk.id.to_owned());
        return Ok(());
    }

    /// Returns true if this group has multiple items (actual sibering occurred)
    pub fn has_siblings(&self) -> bool {
        self.sebering_id_list.len() > 1
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

        // if self.id == "bafyreiemw4vij6drxf24xikw5dgrylnb5aqha2nvbbmrwffotzxkt5jwlm" {
        //     println!("resolve_nest_children : {:#?}", self);
        // }
        // 696b1639e212b5ce0180b80d

        if let Some(root_blk) = tmp_cache_contents.get(&self.id) {
            tmp_text_mark_and_number.push(PageTextListSet {
                style: TextStyle::Title,
                is_header: true,
                start_id: root_blk.id.to_owned(),
                start_child: root_blk.children_ids.to_owned().unwrap_or_default(),
                ..PageTextListSet::default()
            });
        }

        let current_layer = 1;

        // First pass: resolve internal children and collect sibering groups
        for id in order_list {
            let ct_blk = tmp_cache_contents.get_mut(id);

            if let Some(blk) = ct_blk {
                self.resolve_internal_children_pipeline(
                    blk,
                    &mut tmp_text_mark_and_number,
                    current_layer,
                );
            }
        }

        // Second pass: group consecutive list items and build final contents
        self.apply_sibering_groups(order_list, &tmp_text_mark_and_number, current_layer);
    }

    /// Apply sibering groups to create grouped list containers in the final output
    fn apply_sibering_groups(
        &mut self,
        order_list: &Vec<String>,
        text_sebering: &PageTextListMainSet,
        layer: usize,
    ) {
        // Get all sibering groups at this layer that have multiple items
        let layer_groups: Vec<&PageTextListSet> = text_sebering
            .iter()
            .filter(|g| g.layer == layer && g.sebering_id_list.len() >= 1)
            .collect();

        // Build a set of IDs that are part of sibering groups (not the first item)
        let mut skip_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        for group in layer_groups.iter() {
            // Skip all items except the first one - they'll be grouped with the first
            for id in group.sebering_id_list.iter().skip(1) {
                skip_ids.insert(id.clone());
            }
        }

        // Build the final contents list
        for id in order_list {
            // Skip items that are part of a sibering group (not the first item)
            if skip_ids.contains(id) {
                continue;
            }

            // Check if this id is the start of a sibering group
            let sibering_group = layer_groups.iter().find(|g| g.start_id == *id);

            if let Some(group) = sibering_group {
                if group.sebering_id_list.len() > 1 {
                    // This is the start of a group with siblings - create a container
                    let container = self.create_list_container(group);
                    self.contents.push(container);
                } else {
                    // Single item, just push it normally
                    if let Some(blk) = self.cache_contents.get(id) {
                        self.contents.push(blk.to_owned());
                    }
                }
            } else {
                // Not part of any sibering group, push normally
                if let Some(blk) = self.cache_contents.get(id) {
                    self.contents.push(blk.to_owned());
                }
            }
        }
    }

    /// Create a container ContentBlock that groups multiple consecutive list items
    fn create_list_container(&self, group: &PageTextListSet) -> ContentBlock {
        let mut items: Vec<TextItem> = vec![];

        for id in group.sebering_id_list.iter() {
            if let Some(blk) = self.cache_contents.get(id) {
                let item = TextItem::to_subitem(blk);
                items.push(item);
            }
        }

        // Get the first block to use as the base for the container
        let first_blk = self.cache_contents.get(&group.start_id);

        if let Some(base_blk) = first_blk {
            let mut container = base_blk.clone();

            // Update the component_attr to include all items
            if let ComponentAttrType::Text(ref mut text_attr) = container.component_attr {
                text_attr.items = Some(items);
            }

            // Update children_ids to include all sibering IDs
            container.children_ids = Some(group.sebering_id_list.clone());

            container
        } else {
            // Fallback: create a minimal container
            ContentBlock {
                id: group.start_id.clone(),
                order: group.start,
                component_attr: ComponentAttrType::Text(TextComponentAttr {
                    style: group.style.clone(),
                    items: Some(items),
                    ..TextComponentAttr::default()
                }),
                ..ContentBlock::default()
            }
        }
    }

    fn resolve_internal_children_pipeline(
        &mut self,
        current_block: &mut ContentBlock,
        text_sebering: &mut PageTextListMainSet,
        current_layer: usize,
    ) {
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

        if attr.style != TextStyle::Marked
            && attr.style != TextStyle::Numbered
            && attr.style != TextStyle::Toggle
        {
            return;
        }
        // println!(in );

        let lk_children_ids = current_block.children_ids.as_ref().unwrap();
        let mut cached = self.cache_contents.to_owned();

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
            // Try to find an existing sibering group that this block can continue
            let sebering = text_sebering.iter_mut().find(|p| {
                p.can_continue_sibering(&attr.style, &cloned_curr_blk.order, &current_layer)
            });

            if let Some(existing_group) = sebering {
                // This block continues an existing sibering group
                let _ = existing_group.push_next(&cloned_curr_blk);
            } else {
                // Start a new sibering group
                text_sebering.push(PageTextListSet {
                    style: attr.style.clone(),
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

    /// Transform nested children within a text block (for Toggle, Marked, Numbered with children)
    fn transform_text_list(
        &mut self,
        current_block: &mut ContentBlock,
        text_sebering: &mut PageTextListMainSet,
        current_layer: usize,
    ) {
        if text_sebering.is_empty() {
            return;
        }
        let cached = self.cache_contents.to_owned();

        // Find sibering groups that are children of this block (next layer down)
        let child_layer_groups: Vec<&PageTextListSet> = text_sebering
            .iter()
            .filter(|p| {
                p.layer == current_layer + 1
                    && current_block
                        .children_ids
                        .as_ref()
                        .is_some_and(|f| f.contains(&p.start_id))
            })
            .collect();

        if !child_layer_groups.is_empty() {
            // Build a set of IDs that should be skipped (non-first items in sibering groups)
            let mut skip_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
            for group in child_layer_groups.iter() {
                for id in group.sebering_id_list.iter().skip(1) {
                    skip_ids.insert(id.clone());
                }
            }

            let mut items: Vec<TextItem> = vec![];

            if let ComponentAttrType::Text(text) = &mut current_block.component_attr {
                if let Some(child_ids) = current_block.children_ids.to_owned() {
                    for child_id in child_ids.iter() {
                        // Skip non-first items in sibering groups
                        if skip_ids.contains(child_id) {
                            continue;
                        }

                        // Check if this child starts a sibering group
                        let child_group =
                            child_layer_groups.iter().find(|g| g.start_id == *child_id);

                        if let Some(group) = child_group {
                            if group.sebering_id_list.len() > 1 {
                                // Create a container for the sibering group
                                let mut group_items: Vec<TextItem> = vec![];
                                for gid in group.sebering_id_list.iter() {
                                    if let Some(blk) = cached.get(gid) {
                                        group_items.push(TextItem::to_subitem(blk));
                                    }
                                }
                                items.push(TextItem::create_container(&group_items));
                            } else {
                                // Single item
                                if let Some(blk) = cached.get(child_id) {
                                    items.push(TextItem::to_subitem(blk));
                                }
                            }
                        } else {
                            // Not part of any sibering group
                            if let Some(blk) = cached.get(child_id) {
                                items.push(TextItem::to_subitem(blk));
                            }
                        }
                    }
                }
                text.items = Some(items);
            }
        }

        // Update the cache with the modified block
        let new_block = current_block.clone();
        self.cache_contents.remove(&current_block.id);
        self.cache_contents
            .insert(new_block.id.to_owned(), new_block);
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
            _ => return,
        }

        // Skip if items are already populated (avoid duplicates)
        if attr.items.as_ref().is_some_and(|items| !items.is_empty()) {
            return;
        }

        let Some(lk_children_ids) = current_block.children_ids.as_ref() else {
            return;
        };
        let lk_children_ids = lk_children_ids.clone();
        let mut cached = self.cache_contents.clone();

        // Track which child IDs have been added to prevent duplicates
        let mut added_ids: std::collections::HashSet<String> = std::collections::HashSet::new();

        for child_id in lk_children_ids.iter() {
            // Skip if already added
            if added_ids.contains(child_id) {
                continue;
            }

            if let Some(child_blk) = cached.get_mut(child_id) {
                if child_blk.children_ids.is_some() {
                    self.resolve_internal_children_pipeline(
                        child_blk,
                        text_sebering,
                        current_layer + 1,
                    );
                }

                let layout_item = LayoutItem::from_content_block(child_blk);
                if let Err(e) = attr.push_item(layout_item) {
                    eprintln!("Error adding layout item: {:?}", e);
                } else {
                    added_ids.insert(child_id.clone());
                }
            }
        }

        // Update the cache with the modified block (including the items)
        let updated_block = current_block.clone();
        self.cache_contents
            .insert(updated_block.id.clone(), updated_block);
    }
}
