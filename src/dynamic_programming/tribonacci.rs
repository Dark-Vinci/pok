use std::collections::HashMap;
use super::Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        Self::tribonacci_helper(n, &mut HashMap::<i32, i32>::new())
    }

    fn tribonacci_helper(i: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if memo.contains_key(&i) {
            return memo[&i];
        }

        if i == 0 {
            return 0;
        }

        if i <= 2 {
            return 1;
        }

        let result = Self::tribonacci_helper(i - 1, memo) + Self::tribonacci_helper(i - 2, memo) + Self::tribonacci_helper(i - 3, memo);

        memo.insert(i, result);

        result
    }
}