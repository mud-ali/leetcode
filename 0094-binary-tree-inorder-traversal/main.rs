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
    fn process_node(noe : Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        let binding = noe.unwrap();
        let node = binding.borrow();
        if !node.left.is_none() {
            Self::process_node(node.left.clone(), list);
        }
        list.push(node.val);
        if !node.right.is_none() {
            Self::process_node(node.right.clone(), list);
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut trav = Vec::<i32>::new();
        let mut currentNode = root;

        if (!currentNode.is_none()) {
            Self::process_node(currentNode, &mut trav);
        }

        trav
    }
}