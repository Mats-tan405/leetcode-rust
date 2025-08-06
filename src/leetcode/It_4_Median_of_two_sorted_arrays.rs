/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */
pub struct Solution;
// @lc code=start
pub fn merge_sort(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::with_capacity(nums1.len() + nums2.len());
    let (mut i,mut j) = (0,0);
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }        
    }
    // 合并剩余元素
    if i < nums1.len() {
        merged.extend(&nums1[i..]);
    } else {
        merged.extend(&nums2[j..]);
    }
    merged
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 归并排序求中位数
        let arrays = merge_sort(nums1, nums2);
        let mut carry = 0.0;
        if arrays.len()%2 == 0 {
            carry = (arrays[arrays.len()/2] + arrays[arrays.len()/2 - 1]) as f64 / 2.0;
        } else {
            carry = arrays[arrays.len()/2] as f64;
        }
        carry
    }
}
// @lc code=end
#[cfg(test)]
mod solution_test {
    use crate::leetcode::{It_4_Median_of_two_sorted_arrays::Solution};

    #[test]
    pub fn find_median_sorted_arrays_test() {
        let carry = Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]);
        println!("result:{carry}");
    }
}

