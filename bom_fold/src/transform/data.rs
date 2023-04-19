use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Abstract input data, extracted from some flat format like excel or CSV.
#[derive(Debug, PartialEq)]
pub struct FlatData<'a> {
    pub keys: Vec<Cow<'a, str>>,
    pub records: Vec<Vec<Value<'a>>>,
}

/// Individual parsed from the flat file.
#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize)]
pub enum Value<'a> {
    Text(Cow<'a, str>),
    Number(f64),
}

impl Value<'_> {
    /// Constructs `Value::Text` from unowned value.
    pub fn text(text: &str) -> Value {
        Value::Text(Cow::from(text))
    }

    /// Constructs `Value::Text` from owned value or clones if necessary.
    pub fn text_owned<S: ToString>(text: S) -> Value<'static> {
        Value::Text(Cow::from(text.to_string()))
    }
}

/// Possible types that a value can take.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize)]
pub enum ValueType {
    Text,
    Number,
}

/// Abstract item hierarchy, the output of folding.
#[derive(Debug, Default, PartialEq)]
pub struct FoldedData<'a> {
    /// Names of the attributes that stored positionally on all nodes in the hierarchy.
    pub attribute_keys: &'a [Cow<'a, str>],

    /// All of the nodes that do not have parents.
    pub top_level_nodes: Vec<Node<'a>>,
}

/// Single element in the abstract item hierarchy.
#[derive(Debug, Default, PartialEq)]
pub struct Node<'a> {
    /// Values associated with this node, keyed by the `attribute_keys` in the owning `FoldedData`.
    pub attributes: &'a [Value<'a>],

    /// Descendent nodes.
    pub children: Vec<Node<'a>>,
}
