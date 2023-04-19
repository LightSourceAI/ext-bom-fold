use crate::transform::{absolute_parent_locator::AbsoluteParentLocator, data::ValueType};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Rules {
    pub type_mapping: Option<HashMap<String, ValueType>>,
    pub child_identification_policy: ChildIdentificationPolicy,
    /// Determines the format for the output.
    pub output_rules: OutputRules,
}

/// There are two ways we can identify children:
/// 1. There's some level key that indicates BOM depth. Any time the bom depth level increases, it
///    indicates that all subsequent elements are children of the most recent parent.
/// 2. We can do an exact lookup in the list to find the parent based on some key comparison.
#[derive(Deserialize)]
pub enum ChildIdentificationPolicy {
    OrderedLevelKey(String),
    Absolute(AbsoluteParentLocator),
}

#[derive(Deserialize)]
pub enum OutputRules {
    ItemSync(ItemSyncFormatRules),
}

#[derive(Deserialize)]
pub struct ItemSyncFormatRules {
    pub id_key: String,

    /// If `None` then the id is reused for the name.
    pub name_key: Option<String>,

    /// If `None` then quantity will default to 1.
    pub quantity_key: Option<String>,
}
