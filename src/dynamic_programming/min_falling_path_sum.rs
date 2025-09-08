use std::cmp::min;
use super::Solution;

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (l, ll) = (grid.len(), grid[0].len());

        let mut memo = vec![vec![i32::MAX; ll]; l];
        let mut result = i32::MAX;

        for j in 0..l {
            result = min(result, Self::min_falling_path_sum_helper(0, j as i32, l as i32, ll as i32, &mut memo, &grid))
        }

        result
    }

    fn min_falling_path_sum_helper(i: i32, j: i32, l: i32, ll: i32, memo: &mut Vec<Vec<i32>>, mat: &Vec<Vec<i32>>) -> i32 {
        if i == l || j == ll {
            return i32::MAX;
        }

        if memo[i as usize][j as usize] != i32::MAX {
            return memo[i as usize][j as usize];
        }

        if i == l - 1 {
            memo[i as usize][j as usize] = i32::MAX;
            return memo[i as usize][j as usize];
        }

        let mut result = i32::MAX;

        for k in 0..ll {
            if k != j {
                result = min(result, Self::min_falling_path_sum_helper(i + 1, k,  l, ll,  memo, mat));
            }
        }

        memo[i as usize][j as usize] = mat[i as usize][j as usize] + result;

        memo[i as usize][j as usize]
    }
}