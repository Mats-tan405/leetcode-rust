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
        let s_len = s.len() as i32;
        let (mut i, mut j) = (0,1);
        let (mut max_len,mut begin) = (0,0);
        // 极端情况处理--(偶数中心)
        (begin,max_len) = Solution::longest_palindrome_core(&s, s_len/2-1, s_len/2, begin, max_len);
        if max_len == s_len { return Solution::get_substring_unchecked(&s, begin as usize, (max_len) as usize);}
        // 极端情况处理--(奇数中心)
        (begin,max_len) = Solution::longest_palindrome_core(&s, s_len/2, s_len/2 + 1, begin, max_len);
        if max_len == s_len { return Solution::get_substring_unchecked(&s, begin as usize, (max_len) as usize);}

        // 一般计算流程
        for (index,c) in s.chars().enumerate() {
            // (i,j) = binary_move(); // 中心移动函数
            (begin,max_len) = Solution::longest_palindrome_core(&s, i, j, begin, max_len);
            i+=1;
            j+=1;
        }
        Solution::get_substring_unchecked(&s, begin as usize, (max_len) as usize)
    }
    pub fn longest_palindrome_core(s:&String, i:i32, j:i32, begin:i32, max_len:i32) -> (i32,i32) {
        let mut max_len = max_len;
        let mut begin = begin;
        let mut extend = 0;
        let mut s_len = s.len() as i32;
        while Solution::is_palindrome(&s, i - extend, j + extend) && (i-extend)>=0 && (j+extend)<=s_len {
            if max_len < (2 + extend*2) {
                max_len = 2 + extend*2;
                begin = i - extend;
            }
            extend += 1;
        }
        // i += 1;
        extend = 0;
        while Solution::is_palindrome(&s, i-extend, i+extend) && (i-extend)>=0 && (i+extend)< s_len {
            if max_len < (1 + extend*2) {
                // 最长回文串变更
                max_len = 1 + extend*2;
                begin = i - extend;
            }
            extend += 1;
        }
        (begin, max_len)        
    }

    pub fn is_palindrome(s:&String, i:i32, j:i32) -> bool {
        // 将字符串转换为字符切片，并处理索引边界
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        // 转换索引为 usize 并校验合法性
        let left = i as usize;
        let right = j as usize;

        // if left > right || left >= len || right >= len {
        //     return false; // 无效索引范围
        // }
        if left > right || left >= len || right >= len {
            return false; // 无效索引范围
        }

        // 双指针向中间移动比较字符
        let mut l = left;
        let mut r = right;
        while l < r {
            if chars[l] != chars[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }

    fn get_substring_unchecked(s: &String, i: usize, len: usize) -> String {
        let start = s.char_indices().nth(i).unwrap().0; // 强制解包，可能 panic
        let end = start + len;
        s[start..end].to_string()
    }
    
}
// @lc code=end
    #[test]
    fn get_substring_unchecked_test() {
        let s:String = "babad".to_string();
        let s2 = Solution::get_substring_unchecked(&s, 0, 3);
        println!("{:?}",s2);
    }

    #[test]
    fn longest_palindrome_test1() {
        let s:String = "babad".to_string();
        let s2 = Solution::longest_palindrome(s);
        assert_eq!(s2,"aba");
    }

    #[test]
    fn longest_palindrome_test2() {
        let s:String = "a".to_string();
        let s2 = Solution::longest_palindrome(s);
        assert_eq!(s2,"a");
    }

    #[test]
    fn longest_palindrome_test3() {
        // 这是我目前算法时间复杂度的极端情况，这是由于每一个位置都会完全计算，且奇数、偶数判断都会完全进行。
        /*
            要对这个做出优化的话，需要让中心窗格更高效的移动，比如以“二分法”的形式移动，这样就能在“最佳”位置开始判断，同时也能遍历所有点
         */
        let s:String = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string();
        let s2 = Solution::longest_palindrome(s);
        assert_eq!(s2,"1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");
    }

    #[test]
    fn longest_palindrome_test4() {
        let s:String = "baabbbbbbaad".to_string();
        let s2 = Solution::longest_palindrome(s);
        assert_eq!(s2,"aabbbbbbaa");
    }

    #[test]
    fn longest_palindrome_test5() {
        let s:String = "cccccc".to_string();
        let s2 = Solution::longest_palindrome(s);
        assert_eq!(s2,"cccccc");
    }

    #[test]
    fn longest_palindrome_test6() {
        let s:String = "cccccccccccc".to_string();
        let s2 = Solution::longest_palindrome(s);
        assert_eq!(s2,"cccccccccccc");
    }

    #[test]
    fn longest_palindrome_test7() {
        let s:String = "ababababababa".to_string();
        let s2 = Solution::longest_palindrome(s);
        assert_eq!(s2,"ababababababa");
    }
}

