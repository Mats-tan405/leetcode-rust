/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */
struct Solution;
#[derive(PartialEq, Eq,Clone,Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val:i32) ->Self {
        ListNode { val: val, next: None }
    }
}
// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1; // 链表1可编辑
        let mut l2 = l2; // 链表2可编辑
        let mut carry = 0; // 进位器
        let mut dummy = Box::new(ListNode::new(0)); // 结果链表
        let mut current = &mut dummy; // 结果链表指针

        // 链表1未走完、链表2未走完、仍有进位未入链，均继续计算
        while l1.is_some() || l2.is_some() || carry != 0 {
            let (val1,n_l1) = match l1 {
                Some(node) => (node.val,node.next), // 链表取值
                None => (0,None) // 链表停止移动
            };
            let (val2,n_l2) = match l2 {
                Some(node) => (node.val,node.next),
                None => (0,None)
            };
            // 计算值1、值2和上一次计算的进位
            let sum = val1 + val2 + carry;
            carry = sum/10;
            // 存结果的个位数
            let mut node = Box::new(ListNode::new(sum%10));
            current.next = Some(node);
            // 三个链表均向前移动
            current = current.next.as_mut().unwrap();
            l1 = n_l1;
            l2 = n_l2;
        }

        dummy.next
    }
}
// @lc code=end

