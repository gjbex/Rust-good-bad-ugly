use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Deserialize, Serialize)]
pub struct Node<T> {
    pub(crate) value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Deserialize, Serialize)]
pub struct Tree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn left(&self) -> Option<&Node<T>> {
        self.left.as_deref()
    }

    pub(crate) fn left_mut(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.left
    }

    pub fn right(&self) -> Option<&Node<T>> {
        self.right.as_deref()
    }

    pub(crate) fn right_mut(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.right
    }
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn root(&self) -> Option<&Node<T>> {
        self.root.as_deref()
    }

    pub(crate) fn root_mut(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.root
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree::new()
    }
}

impl<T> Tree<T> {
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn nr_nodes(&self) -> usize {
        fn count_nodes<T>(node: &Node<T>) -> usize {
            1 + node.left().map_or(0, count_nodes) + node.right().map_or(0, count_nodes)
        }
        self.root().map_or(0, count_nodes)
    }
}

impl<T: Display> Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn fmt_node<T: Display>(
            node: &Node<T>,
            f: &mut fmt::Formatter<'_>,
            depth: usize,
            label: &str,
        ) -> fmt::Result {
            for _ in 0..depth {
                write!(f, "  ")?; // Indent based on depth
            }
            writeln!(f, "{label}: {}", node.value)?;
            if let Some(left) = node.left() {
                fmt_node(left, f, depth + 1, "L")?;
            }
            if let Some(right) = node.right() {
                fmt_node(right, f, depth + 1, "R")?;
            }
            Ok(())
        }

        if let Some(root) = self.root() {
            fmt_node(root, f, 0, "root")
        } else {
            write!(f, "Empty Tree")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;
    use serde_json::json;

    #[test]
    fn display_labels_tree_structure_from_deserialized_tree() {
        let tree_json = json!({
            "root": {
                "value": 2,
                "left": {
                    "value": 1,
                    "left": null,
                    "right": null
                },
                "right": {
                    "value": 3,
                    "left": null,
                    "right": null
                }
            }
        });
        let tree: Tree<i32> = serde_json::from_value(tree_json.clone()).unwrap();

        assert_eq!(format!("{tree}"), "root: 2\n  L: 1\n  R: 3\n");
        assert_eq!(serde_json::to_value(&tree).unwrap(), tree_json);
    }
}
