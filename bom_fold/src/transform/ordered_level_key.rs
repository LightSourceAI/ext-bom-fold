use crate::transform::data::{FlatData, FoldedData, Node, Value};
use error::{Error, Result};
use std::cmp::Ordering;

/// Folds the flat data using a parent node key.
pub fn fold<'data>(flat_data: &'data FlatData, level_key: &str) -> Result<FoldedData<'data>> {
    if flat_data.records.is_empty() || flat_data.keys.is_empty() {
        return Ok(FoldedData::default());
    }

    let level_key_index = flat_data.keys.iter().position(|item| item == level_key);
    let level_key_index = level_key_index
        .ok_or_else(|| Error::invalid_argument("Couldn't find level key in the flat data keys"))?;

    // The root nodes in the data if there are multiple top level boms.
    let mut top_level_nodes = Vec::with_capacity(flat_data.records.len());

    // Live nodes that have not been yet had all their children assigned nor assigned to their
    // parents.
    let mut working_node_stack = Vec::with_capacity(flat_data.records.len());

    for record in flat_data.records.iter() {
        let current_record_level = &record[level_key_index];

        if !working_node_stack.is_empty() {
            unwind_working_stack(
                &mut working_node_stack,
                &mut top_level_nodes,
                current_record_level,
            );
        }
        working_node_stack.push(LevelNode {
            level: current_record_level.clone(),
            node: Node { attributes: record, children: Vec::new() },
        });
    }
    unwind_working_stack_unconditionally(&mut working_node_stack, &mut top_level_nodes);
    Ok(FoldedData { top_level_nodes, attribute_keys: &flat_data.keys })
}

/// Stores the level alongside the node for convenience.
struct LevelNode<'a> {
    level: Value<'a>,
    node: Node<'a>,
}

/// Pops the working stack for all of the "done" nodes (we know there are no more children because
/// we're now going back up in levels).
fn unwind_working_stack<'a>(
    working_node_stack: &mut Vec<LevelNode<'a>>,
    top_level_nodes: &mut Vec<Node<'a>>,
    current_record_level: &Value,
) {
    loop {
        // Fetch current working node and also check/return if the list is empty.
        let working_node_level = match working_node_stack.last() {
            Some(node) => &node.level,
            None => return,
        };
        // Finalize the current working node if its not the parent of the next node.
        match working_node_level.partial_cmp(current_record_level) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => {
                finalize_working_node(working_node_stack, top_level_nodes)
            }
            _ => return,
        }
    }
}

fn unwind_working_stack_unconditionally<'a>(
    working_node_stack: &mut Vec<LevelNode<'a>>,
    top_level_nodes: &mut Vec<Node<'a>>,
) {
    while !working_node_stack.is_empty() {
        finalize_working_node(working_node_stack, top_level_nodes);
    }
}

fn finalize_working_node<'a>(
    working_node_stack: &mut Vec<LevelNode<'a>>,
    top_level_nodes: &mut Vec<Node<'a>>,
) {
    let popped = match working_node_stack.pop() {
        Some(p) => p,
        None => return,
    };
    match working_node_stack.last_mut() {
        Some(parent) => parent.node.children.push(popped.node),
        None => top_level_nodes.push(popped.node),
    }
}

#[cfg(test)]
mod tests {
    use crate::transform::{FlatData, FoldedData, Node, Value};
    use pretty_assertions::assert_eq;
    use std::borrow::Cow;

    fn test_case(key: &str, input: &FlatData, output: &FoldedData) {
        assert_eq!(&super::fold(&input, key).unwrap(), output)
    }

    fn keys() -> Vec<Cow<'static, str>> {
        ["level", "foo"].into_iter().map(Cow::from).collect()
    }

    #[test]
    fn degenerate() {
        let input =
            FlatData { keys: keys(), records: vec![vec![Value::text("1"), Value::text("foo")]] };
        let output = FoldedData {
            top_level_nodes: vec![Node { attributes: &input.records[0], children: Vec::new() }],
        };
        test_case("level", &input, &output);
    }

    #[test]
    fn single_root_single_layer() {
        let input = FlatData {
            keys: keys(),
            records: vec![
                vec![Value::text("1"), Value::text("foo")],
                vec![Value::text("1.1"), Value::text("foo")],
            ],
        };
        let output = FoldedData {
            top_level_nodes: vec![Node {
                attributes: &input.records[0],
                children: vec![Node { attributes: &input.records[1], children: Vec::new() }],
            }],
        };
        test_case("level", &input, &output);
    }

    #[test]
    fn single_root_single_layer_number() {
        let input = FlatData {
            keys: keys(),
            records: vec![
                vec![Value::Number(1.0), Value::text("foo")],
                vec![Value::Number(1.1), Value::text("foo")],
            ],
        };
        let output = FoldedData {
            top_level_nodes: vec![Node {
                attributes: &input.records[0],
                children: vec![Node { attributes: &input.records[1], children: Vec::new() }],
            }],
        };
        test_case("level", &input, &output);
    }

    #[test]
    fn multi_root_single_layer() {
        let input = FlatData {
            keys: keys(),
            records: vec![
                vec![Value::text("1"), Value::text("1")],
                vec![Value::text("1.1"), Value::text("2")],
                vec![Value::text("1"), Value::text("3")],
                vec![Value::text("1.1"), Value::text("4")],
            ],
        };
        let output = FoldedData {
            top_level_nodes: vec![
                Node {
                    attributes: &input.records[0],
                    children: vec![Node { attributes: &input.records[1], children: Vec::new() }],
                },
                Node {
                    attributes: &input.records[2],
                    children: vec![Node { attributes: &input.records[3], children: Vec::new() }],
                },
            ],
        };
        test_case("level", &input, &output);
    }

    #[test]
    fn single_root_multi_layer() {
        let input = FlatData {
            keys: keys(),
            records: vec![
                vec![Value::text("1"), Value::text("0")],
                vec![Value::text("1.1"), Value::text("1")],
                vec![Value::text("1.1"), Value::text("2")],
                vec![Value::text("1.2"), Value::text("3")],
                vec![Value::text("1.1"), Value::text("4")],
            ],
        };
        let output = FoldedData {
            top_level_nodes: vec![Node {
                attributes: &input.records[0],
                children: vec![
                    Node { attributes: &input.records[1], children: Vec::new() },
                    Node {
                        attributes: &input.records[2],
                        children: vec![Node {
                            attributes: &input.records[3],
                            children: Vec::new(),
                        }],
                    },
                    Node { attributes: &input.records[4], children: Vec::new() },
                ],
            }],
        };
        test_case("level", &input, &output);
    }
}
