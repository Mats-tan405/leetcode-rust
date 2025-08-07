/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let (mut begin, mut max_len) = (0,1);
        let mut dp = vec![vec![false;len];len];
        let chars:Vec<char> = s.chars().collect();
        for j in 0..len {
            let c1 = chars[j];
            for i in 0..j {
                let c2 = chars[i];
                if (c1 == c2) {
                    if ( j - i <= 2) {
                        dp[i][j] = true;
                    } else if dp[i+1][j-1] {
                        dp[i][j] = dp[i+1][j-1];
                    }
                    if dp[i][j] && (j - i + 1 > max_len) {
                        max_len = j - i + 1;
                        begin = i;
                    }
                }
            }
        }

        s[begin..begin+max_len].to_string()
    }
}
// @lc code=end

