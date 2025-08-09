// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {

    fn helper(node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        let binding = node.unwrap();
        let node = binding.borrow();
        let mut left_depth: i32 = depth;
        let mut right_depth: i32 = depth;
        if !node.left.is_none() {
            left_depth = Self::helper(node.left.clone(), depth+1);
        }
        if !node.right.is_none() {
           right_depth = Self::helper(node.right.clone(), depth+1);
        }
        if node.right.is_none() && node.left.is_none() {
            return depth;
        }
        return left_depth.max(right_depth);
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if !root.is_none() {
            return Self::helper(root,1);
        }
        0
    }
}