use std::cmp::min;
use super::Solution;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let pattern = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
            [-1, 0, -1],
        ];

        let dirs = [
            [1, -2],
            [2, -1],
            [-1, -2],
            [-2, -1],
            [-2, 1],
            [-1, 2],
            [1, 2],
            [2, 1],
        ];

        let mut result = 0;
        let mut memo = [[[-1; 5001]; 3]; 4];

        for (i, pat) in pattern.iter().enumerate() {
            for (j, &p) in pat.iter().enumerate() {
                if p != -1 {
                    result = (((result + Self::knight_dialer_helper(j as i32, i as i32, 1, n, &pattern, &dirs, &mut memo)) as i64) % Self::MOD) as i32;
                }
            }
        }

        result
    }

    fn knight_dialer_helper(x: i32, y: i32, z: i32, n: i32, pattern: &[[i32; 3]; 4], dirs: &[[i32;2];8], memo: &mut [[[i32; 5001]; 3]; 4]) -> i32 {
        if x < 0 || y < 0 || y >= pattern.len() as i32 || x >= pattern[0].len() as i32 || pattern[x as usize][y as usize] == -1  {
            return 0;
        }

        if memo[x as usize][y as usize][z as usize] != -1 {
            return memo[x as usize][y as usize][z as usize];
        }

        if z == n {
            return 1;
        }

        let (zz, mut result) = (z + 1, 0);

        for dir in dirs.iter() {
            let (xx, yy) = (x + dir[0], y + dir[1]);
            result = (((result + Self::knight_dialer_helper(xx, yy, zz, n, pattern, dirs, memo)) as i64) % Self::MOD) as i32;
        }

        memo[x as usize][y as usize][z as usize] = result;

        memo[x as usize][y as usize][z as usize]
    }
}