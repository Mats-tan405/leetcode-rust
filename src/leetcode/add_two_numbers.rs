/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */
struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
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
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let (val1,n_l1) = match l1 {
                Some(node) => (node.val,node.next),
                None => (0,None),
            };
            let (val2,n_l2) = match l2 {
                Some(node) => (node.val,node.next),
                None => (0,None)
            };
            let sum = val1 + val2 + carry;
            carry = sum/10;
            let mut node = Box::new(ListNode::new(sum%10));
            current.next = Some(node);
            current = current.next.as_mut().unwrap();
            
            l1 = n_l1;
            l2 = n_l2;
        } 

        dummy.next
    }
}
// @lc code=end

