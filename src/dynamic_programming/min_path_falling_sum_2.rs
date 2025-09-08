use std::cmp::min;
use super::Solution;

impl Solution {
    pub fn min_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let (l, ll, mut result) = (matrix.len(), matrix[0].len(), i32::MAX);

        let mut memo = vec![vec![i32::MAX; ll]; l];

        for j in 0..ll {
            result = min(result, Self::min_path_sum_helper(0, j as i32, l as i32, ll as i32, &mut memo, &matrix))
        }

        result
    }

    fn min_path_sum_helper(i: i32, j: i32, l: i32, ll: i32, memo: &mut Vec<Vec<i32>>, mat: &Vec<Vec<i32>>) -> i32 {
        if i == ll || j == ll {
            return i32::MAX;
        }

        if memo[i as usize][j as usize] == i32::MAX {
            return memo[i as usize][j as usize];
        }

        if i == l - 1 {
            memo[i as usize][j as usize] = mat[i as usize][j as usize];
            return memo[i as usize][j as usize];
        }

        let (right, left, down) = (Self::min_path_sum_helper(i + 1, j + 1, l, ll, memo, mat), Self::min_path_sum_helper(i + 1, j - 1, l, ll, memo, mat), Self::min_path_sum_helper(i + 1, j, l, ll, memo, mat));

        memo[i as usize][j as usize] = mat[i as usize][j as usize] + (min(right, min(left, down)) as i64 % Self::MOD) as i32;

        memo[i as usize][j as usize]
    }
}