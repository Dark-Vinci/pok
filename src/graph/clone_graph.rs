use std::collections::HashMap;
use crate::graph::{Node, Solution};

impl Solution {
    fn clone_graph(node: Option<Node>) -> Option<Node> {
        Self::clone_helper(node, &mut HashMap::new())
    }

    fn clone_helper(curr: Option<Node>, db: &mut HashMap<i32, Option<Node>>) -> Option<Node> {
        if curr.is_none() {
            return None;
        }

        let curr = curr.unwrap();

        let mut clone = Node::new(curr.value, Vec::with_capacity(curr.neighbors.len()));

        db.insert(curr.value, Some(clone.clone()));

        for (i, val) in curr.neighbors.into_iter().enumerate() {
            if let Some(present) = db.get(&curr.value) {
                clone.neighbors.insert(i, present.clone());
            } else {
                let recurse = Self::clone_helper(val, db);
                clone.neighbors.insert(i, recurse);
            }
        }

        Some(clone)
    }
}