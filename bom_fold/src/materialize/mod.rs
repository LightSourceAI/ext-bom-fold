//! Converts the in memory representation to serialized format.

use crate::{FoldedData, ItemSyncFormatRules, Node, Value};
use error::{Error, Result};
use serde::Serialize;
use std::cmp::Ordering;

/// Serializable output format compatible with the "item sync" input flat file.
#[derive(Debug, Serialize)]
pub struct ItemSyncFormat<'a> {
    pub boms: Vec<BomRecord<'a>>,
    pub bom_entries: Vec<BomEntryRecord<'a>>,
}

/// Serializable output meant to populate the "BOMs" sheet of the item sync input flat file.
#[derive(Debug, Serialize)]
pub struct BomRecord<'a> {
    id: &'a Value<'a>,
    name: &'a Value<'a>,
}

/// Serializable output meant to populate the "BOM entries" sheet of the item sync input flat file.
#[derive(Debug, Serialize)]
pub struct BomEntryRecord<'a> {
    bom_id: &'a Value<'a>,
    entry_type: &'a str,
    entry_id: &'a Value<'a>,
    quantity: f64,
}

impl ItemSyncFormat<'_> {
    /// Converts the folded data into a serializable format compatible with the item sync flat
    /// file.
    pub fn format_item_sync<'a>(
        folded_data: &'a FoldedData,
        rules: &ItemSyncFormatRules,
    ) -> Result<ItemSyncFormat<'a>> {
        let indices = AttributeIndices::new(folded_data, rules)?;

        let mut boms = Vec::new();
        let mut bom_entries = Vec::new();
        for node in folded_data.top_level_nodes.iter() {
            Self::recursively_make_records(
                &mut boms,
                &mut bom_entries,
                &indices,
                node,
                /*parent_node=*/ None,
            )?;
        }
        boms.sort_unstable_by(|lhs, rhs| lhs.id.partial_cmp(rhs.id).unwrap_or(Ordering::Equal));
        boms.dedup_by_key(|b| b.id);
        Ok(ItemSyncFormat { boms, bom_entries })
    }

    /// Adds boms to the bom list and entries to the entry list via DFS.
    /// If an item has any children, it is assumed to be a "sub-bom". If it has no children it is
    /// assumed to be a "part".
    fn recursively_make_records<'a>(
        boms: &mut Vec<BomRecord<'a>>,
        bom_entries: &mut Vec<BomEntryRecord<'a>>,
        indices: &AttributeIndices,
        node: &'a Node,
        parent_node_id: Option<&'a Value<'a>>,
    ) -> Result<()> {
        let entry_type = match node.children.is_empty() {
            true => "part",
            false => "sub-bom",
        };
        let node_id = node
            .attributes
            .get(indices.id)
            .ok_or_else(|| Error::invalid_argument("Node is missing id"))?;
        // Add as child of parent node.
        if let Some(parent_node_id) = parent_node_id {
            bom_entries.push(BomEntryRecord {
                bom_id: parent_node_id,
                entry_type,
                entry_id: node_id,
                quantity: indices
                    .quantity
                    .and_then(|index| match node.attributes.get(index) {
                        Some(Value::Number(n)) => Some(*n),
                        _ => None,
                    })
                    .unwrap_or(1.0),
            });
        }
        if node.children.is_empty() {
            return Ok(());
        }

        let name = node
            .attributes
            .get(indices.name)
            .ok_or_else(|| Error::invalid_argument("Unable to find name field in BOM node."))?;
        boms.push(BomRecord { id: node_id, name });
        for child in node.children.iter() {
            Self::recursively_make_records(boms, bom_entries, indices, child, Some(node_id))?;
        }
        Ok(())
    }
}

/// Positions of relevant attributes in the Node attribute vectors.
struct AttributeIndices {
    id: usize,
    name: usize,
    quantity: Option<usize>,
}

impl AttributeIndices {
    fn new(folded_data: &FoldedData, rules: &ItemSyncFormatRules) -> Result<Self> {
        let id_key = &rules.id_key;
        let id_index = folded_data
            .attribute_keys
            .iter()
            .position(|a| a == id_key)
            .ok_or_else(|| Error::invalid_argument("id_key not found in folded data"))?;
        let name_index = rules
            .name_key
            .as_ref()
            .and_then(|name_key| folded_data.attribute_keys.iter().position(|a| a == name_key))
            .unwrap_or(id_index);
        let quantity_index = rules
            .quantity_key
            .as_ref()
            .and_then(|key| folded_data.attribute_keys.iter().position(|a| a == key));
        Ok(Self { id: id_index, name: name_index, quantity: quantity_index })
    }
}
