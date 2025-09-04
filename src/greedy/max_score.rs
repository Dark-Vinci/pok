use crate::greedy::Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut result = 0;
        let mut stk: Vec<u8> = Vec::new();

        let (mut first, mut second) = (("ab", x), ("ba", y));

        if second.1 > first.1 {
            std::mem::swap(&mut first, &mut second);
        }

        for curr in s.bytes() {
            if let Some(&last) = stk.last() {
                if last == first.0.as_bytes()[0] && curr == first.0.as_bytes()[1] {
                    stk.pop();
                    result += first.1;
                    continue;
                }
            }

            stk.push(curr);
        }

        let mut new_stk: Vec<u8> = Vec::new();

        for curr in stk {
            if let Some(&last) = new_stk.last() {
                if last == second.0.as_bytes()[0] && curr == second.0.as_bytes()[1] {
                    new_stk.pop();
                    result += second.1;
                    continue;
                }
            }

            new_stk.push(curr);
        }

        result
    }
}