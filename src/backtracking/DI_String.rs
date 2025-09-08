use std::collections::HashSet;
use crate::backtracking::Solution;

impl Solution {
    const MOD: u32 = 1_000_000_007;
    
    fn dfs(s: String, prev: i32, curr: i32, str: &str, used: &mut HashSet<i32>) -> Vec<String> {
        if curr == s.len() as i32 {
            return vec![str.to_string()];
        }

        let mut result = vec![];

        for n in 1..=9 {
            if used.contains(&n) {
                continue;
            }

            if prev == 0 ||
                (&s.chars().nth((curr - 1) as usize).unwrap() == &'D' && n < prev) ||
                (&s.chars().nth((curr - 1) as usize).unwrap() == &'I' && n > prev)
            {
                used.insert(n);

                let new_str = format!("{}{}", str, n);

                result.extend(Self::dfs(s.clone(), n, curr + 1, &new_str, used));

                used.remove(&n);
            }
        }

        result
    }
    
    pub fn smallest_number(s: String) -> String {
        let mut result = Self::dfs(s, -1, 0, "", &mut HashSet::new());

        result.sort();

        result[0].to_string()
    }
}
