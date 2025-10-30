use super::Solution;

impl Solution {
    fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        let mut memo: Vec<Vec<i8>> = vec![vec![-1; t.len()]; s.len()];

        Self::is_subsequence_dp(0, 0, &s, &t, &mut memo)
    }

    // ITERATIVE APPROACH
    fn is_subsequence1(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        let mut curr = 0;

        for i in 0..t.len() {
            if curr < s.len() && t.chars().nth(i) == s.chars().nth(curr) {
                curr += 1;
            }
        }

        curr == s.len()
    }


    //  RECURSIVE APPROACH(dp)
    fn is_subsequence_dp(curr: i32, indx: i32, s: &String, t: &String, memo: &mut Vec<Vec<i8>>) -> bool {
        let (t_len, s_len) = (t.len() as i32, s.len() as i32);

        if curr >= t_len && indx != s_len {
            return false;
        }

        if indx == s_len {
            return true;
        }

        let mem_result = memo[indx as usize][curr as usize];

        if mem_result != -1 {
            return if mem_result == 0 {
                false
            } else {
                true
            }
        }

        let mut dont = Self::is_subsequence_dp(curr + 1, indx, s, t, memo);

        // we can unwrap safely because of the initial checks [WE DONT EVEN NEED TO UNWRAP]
        if s.chars().nth(indx as usize) == t.chars().nth(curr as usize) {
            dont = dont || Self::is_subsequence_dp(curr + 1, indx + 1, s, t, memo);
        }

        if dont {
            memo[indx as usize][curr as usize] = 1i8;
        } else {
            memo[indx as usize][curr as usize] = 0i8;
        }

        dont
    }
}