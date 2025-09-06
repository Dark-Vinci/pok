use crate::tree::{Solution, Tree};

impl Solution {
    pub fn has_path_sum(root: Option<Box<Tree<i32>>>, target: i32) -> bool {
        Self::has_path_sum_dfs(root, 0, target)
    }

    fn has_path_sum_dfs(root: Option<Box<Tree<i32>>>, mut sum: i32, target: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let root = root.unwrap();

        sum += root.value;

        if root.left.is_none() && root.right.is_none() {
            return sum == target;
        }

        Self::has_path_sum_dfs(root.left, sum, target) || Self::has_path_sum_dfs(root.right, sum, target)
    }
}