use std::cmp::min;
use super::Solution;

impl Solution {
    pub fn min_total_distance(mut robots: Vec<i32>, factories: Vec<Vec<i32>>) -> i64 {
        let mut mod_factories = Vec::new();
        let r = robots.len();

        for fac in factories {
            for i in 0..fac[1] {
                mod_factories.push(fac[0])
            }
        }

        let m = mod_factories.len();
        let mut memo = vec![vec![-1; m]; r + 1];

        robots.sort();
        mod_factories.sort();

        Self::min_total_distance_dfs((r - 1) as i32, (m - 1) as i32, &mut memo, &robots, &mod_factories)
    }

    fn min_total_distance_dfs(r: i32, f: i32, memo: &mut Vec<Vec<i64>>, robots: &Vec<i32>, factories: &Vec<i32>) -> i64 {
        if r < 0 {
            return 0;
        }

        if f < 0 {
            return i64::MAX;
        }

        if memo[r as usize][f as usize] != -1 {
            return memo[r as usize][f as usize];
        }

        let (mut dont, mut doe) = (Self::min_total_distance_dfs(r, f - 1, memo, robots, factories), Self::min_total_distance_dfs(r - 1, f - 1, memo, robots, factories));

        if doe != i64::MAX {
            doe += (factories[f as usize] - robots[r as usize]).abs() as i64;
            dont = min(dont, doe)
        }

        memo[r as usize][f as usize] = dont;

        memo[r as usize][f as usize]
    }
}