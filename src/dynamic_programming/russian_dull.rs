use std::cmp::Ordering;
use std::cmp::max;
use super::Solution;

impl Solution {
    // Time out (TTL, TLE)
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        let l = envelopes.len();

        let mut memo: Vec<Vec<i32>> = vec![vec![-1; l]; l + 1];

        envelopes.sort_by(|a, b| {
            if a[0] > b[0] && a[1] > b[1] {
                Ordering::Greater
            } else {
                a[0].cmp(&b[0])
            }
        });

        Self::dp(l as i32, -1, 0, &mut memo, envelopes)
    }

    fn dp(l: i32, prev: i32, curr: i32, memo: &mut Vec<Vec<i32>>, envelopes: Vec<Vec<i32>>) -> i32 {
        if curr == l {
            return 0
        }

        let v = memo[(prev + 1) as usize][curr as usize];
        if v != -1 {
            return v;
        }

        let mut dont = Self::dp(l, prev, curr + 1, memo, envelopes.clone());

        if prev == -1 ||
            (&envelopes[curr as usize][0usize] > &envelopes[prev as usize][0usize] &&
                &envelopes[curr as usize][1usize] > &envelopes[prev as usize][1usize]) {
            dont = max(dont, 1 + Self::dp(l, curr, curr + 1, memo, envelopes));
        }

        memo[(prev+1) as usize][curr as usize] = dont;

        dont
    }
}
