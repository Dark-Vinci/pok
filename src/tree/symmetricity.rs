use crate::tree::{Solution, Tree};

impl Solution {
    pub fn is_symmetric(root: Option<Box<Tree<i32>>>) -> bool {
        Self::is_syn(root.clone(), root)
    }

    fn is_syn(left: Option<Box<Tree<i32>>>, right: Option<Box<Tree<i32>>>) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }

        if left.is_none() || right.is_none() {
            return false;
        }

        let left = left.unwrap();
        let right = right.unwrap();

        if left.value != right.value {
            return false;
        }

        Self::is_syn(left.right, right.left) && Self::is_syn(left.left, right.right)
    }
}