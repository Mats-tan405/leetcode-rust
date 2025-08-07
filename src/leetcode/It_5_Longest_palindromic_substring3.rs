/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */
#[cfg(test)]
mod solution {
struct Solution;
// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let (mut ans1, mut ans2) = (0,0);
        let mut dp = vec![false;len];
        let chars:Vec<char> = s.chars().collect();
        for i in 0..len {
            for j in 0..=i {
                if i == j {
                    dp[j] = true;
                } else if j + 1 == i {
                    dp[j] = chars[j] == chars[i];
                } else {
                    dp[j] = if chars[j] == chars[i] { dp[j+1] } else { false } ;
                }
                if dp[j] && (i-j > ans2 - ans1) { (ans1,ans2) = (j,i);}
            }
        }

        s[ans1..ans2+1].to_string()
    }
}
// @lc code=end
    #[test]
    fn longest_palindrome_test() {
        let s = "ccc".to_string();
        let s2 = Solution::longest_palindrome(s);
        assert_eq!(s2,"ccc");
    }
}
