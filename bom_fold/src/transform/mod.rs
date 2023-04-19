mod ordered_level_key;

mod absolute_parent_locator;
pub use absolute_parent_locator::*;

mod data;
pub use data::*;

mod rules;
pub use rules::*;

use error::Result;

/// Converts `FlatData` item hierarchy representation into the `FoldedData` representation.
pub fn transform<'data>(flat_data: &'data FlatData, rules: &Rules) -> Result<FoldedData<'data>> {
    match rules.child_identification_policy {
        ChildIdentificationPolicy::OrderedLevelKey(ref key) => {
            ordered_level_key::fold(flat_data, key)
        }
        ChildIdentificationPolicy::Absolute(_) => {
            unimplemented!("Currently don't support absolute parent location")
        }
    }
}
