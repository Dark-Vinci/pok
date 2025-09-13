use super::Solution;

impl Solution {
    pub fn max_coins_coin(nums: Vec<Vec<i32>>, k: i32) -> i32 {
        let nums: Vec<&[i32]> = nums.iter().map(|x| x.as_slice()).collect();

        Self::dfs(0, k, nums.len() as i32, &nums)
    }

    fn dfs(i: i32, k: i32, n: i32, buckets: &[&[i32]]) -> i32 {
        if i == n {
            return 0;
        }

        let dont = Self::dfs(i + 1, k, n, buckets);

        let (mut sum, mut result) = (0, 0);

        for j in 0..std::cmp::min(k as usize, buckets[i as usize].len()) {
            sum += buckets[i as usize][j as usize];

            result = std::cmp::max(result, sum + Self::dfs(i+1, k-(j as i32)-1, n, buckets));
        }

        return std::cmp::max(result, dont);
    }
}