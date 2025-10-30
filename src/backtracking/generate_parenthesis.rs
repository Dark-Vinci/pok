use super::Solution;

impl Solution {
    fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];

        Self::generate_parenthesis_reversed("", n, n, &mut result);

        result
    }

    // TERRIBLE RECURSION BUT WORKS
    fn generate_parenthesis_reversed(curr: &str, l: i32, r: i32, result: &mut Vec<String>) {
        if (l == 0 && r == 0) || r < l {
            return;
        }

        if l == 0 {
            for _ in 0..r {
                let curr = format!("{}{}", curr, ")");

                result.push(curr);
            }

            return;
        }

        let (l_curr, rc_curr) = (format!("{}{}", curr, "("), format!("{}{}", curr, ")"));

        Self::generate_parenthesis_reversed(&l_curr, l - 1, r, result);
        Self::generate_parenthesis_reversed(&rc_curr, l, r - 1, result);
    }
}