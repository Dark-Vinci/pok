use std::cmp::{max, min};
use super::Solution;

impl Solution {
    pub fn max_value_of_coin(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = piles.len();

        let mut memo = vec![vec![-1; (k + 1) as usize]; n];

        Self::max_value_of_coin_dp(0, k, n as i32, &mut memo, &piles)
    }

    fn max_value_of_coin_dp(i: i32, kk: i32, n: i32, memo: &mut Vec<Vec<i32>>, piles: &Vec<Vec<i32>>) -> i32 {
        if i == n {
            return 0;
        }

        if memo[i as usize][kk as usize] != -1 {
            return memo[i as usize][kk as usize];
        }

        let dont = Self::max_value_of_coin_dp(i + 1, kk, n, memo, piles);
        let (mut sum, mut result) = (0, 0);

        for j in 0..min(kk, piles.len() as i32) {
            sum += piles[i as usize][j as usize];
            result = max(result, sum + Self::max_value_of_coin_dp(i + 1, kk - j - 1, n, memo, piles));
        }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      

        memo[i as usize][kk as usize] = max(result, dont);

        memo[i as usize][kk as usize]
    }
}