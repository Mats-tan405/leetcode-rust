/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_index:[i8;128] = [-1;128];
        let mut max_len = 0;
        let mut left = 0;

        for (right,c) in s.chars().enumerate() {
            let idx = c as usize;
            {
                // 窗口滑动
                if last_index[idx] >= left {
                left = last_index[idx] + 1;
                }
                last_index[idx] = right as i8;
            }
            max_len = max_len.max(right as i8 - left + 1);
            // println!("right:{:?}, c:{:?}, left:{:?}, max_len:{:?}",right,c,left,max_len);
        }
        max_len as i32
    }
}
// @lc code=end
#[cfg(test)]
mod substring_test {
    use crate::leetcode::It_3_longest_substring_without_repeating_characters::Solution;

    #[test]
    pub fn length_of_longest_substring_test() {
        let s:String = "pwwkew".to_string();
        let max_len = Solution::length_of_longest_substring(s);
        println!("max_len:{:?}", max_len);
    }
}

