///! This modules is still under construction.
use serde::Deserialize;

/// Locates the parent item in a flat item hierarchy by indexing based on some value.
/// An example could be, each row in the flat item hierarchy has an attribute called "Parent Part
/// Number" which references the "Part Number" attribute of a different item in the flat file.
#[derive(Deserialize)]
pub struct AbsoluteParentLocator {}

// pub struct Computation<'a> {
//     pub inputs: Vec<Expression<'a>>,
//     pub operation: OpName,
// }
//
// pub enum Expression<'a> {
//     Literal(Value<'a>),
//     /// Reference to value in the same row.
//     ReferenceKey(Cow<'a, str>),
//     /// Some computation
//     Computation(Computation<'a>),
// }
//
// pub enum OpName {
//     Equals,
//     Compare,
//     Strip,
//     StripStart,
//     StripEnd,
//     Trim,
//     Concat,
//     ToString,
// }
