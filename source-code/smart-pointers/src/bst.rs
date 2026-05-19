use crate::tree::Node;
pub use crate::tree::Tree;

pub fn insert<T: Ord>(tree: &mut Tree<T>, value: T) {
    insert_node(tree.root_mut(), value);
}

fn insert_node<T: Ord>(link: &mut Option<Box<Node<T>>>, value: T) {
    match link {
        Some(node) if value < node.value => insert_node(node.left_mut(), value),
        Some(node) => insert_node(node.right_mut(), value),
        None => *link = Some(Box::new(Node::new(value))),
    }
}

pub fn search<T: Ord>(tree: &Tree<T>, value: &T) -> bool {
    if let Some(root) = tree.root() {
        search_node(root, value)
    } else {
        false
    }
}

fn search_node<T: Ord>(node: &Node<T>, value: &T) -> bool {
    if &node.value == value {
        true
    } else if value < &node.value {
        if let Some(left) = node.left() {
            search_node(left, value)
        } else {
            false
        }
    } else {
        if let Some(right) = node.right() {
            search_node(right, value)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Tree, insert, search};

    #[test]
    fn empty_tree_does_not_contain_value() {
        let tree = Tree::new();

        assert!(!search(&tree, &42));
    }

    #[test]
    fn inserted_integer_values_can_be_found() {
        let mut tree = Tree::new();

        insert(&mut tree, 42);
        insert(&mut tree, 17);
        insert(&mut tree, 64);

        assert!(search(&tree, &42));
        assert!(search(&tree, &17));
        assert!(search(&tree, &64));
        assert!(!search(&tree, &99));
    }

    #[test]
    fn inserting_duplicate_values_adds_nodes() {
        let mut tree = Tree::new();

        insert(&mut tree, 42);
        insert(&mut tree, 42);

        assert!(search(&tree, &42));
        assert_eq!(tree.nr_nodes(), 2);
    }

    #[test]
    fn inserted_string_values_can_be_found() {
        let mut tree = Tree::new();

        insert(&mut tree, String::from("pear"));
        insert(&mut tree, String::from("apple"));
        insert(&mut tree, String::from("orange"));

        assert!(search(&tree, &String::from("pear")));
        assert!(search(&tree, &String::from("apple")));
        assert!(search(&tree, &String::from("orange")));
        assert!(!search(&tree, &String::from("banana")));
    }
}
