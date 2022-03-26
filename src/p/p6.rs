use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(n) = root {
            let node = n.borrow();
            let right = Solution::tree2str(node.right.clone());
            let right = if right != "" {
                format!("({})", right)
            } else {
                "".to_string()
            };
            let left = Solution::tree2str(node.left.clone());
            let left = if left != "" || right != "" {
                format!("({})", left)
            } else {
                "".to_string()
            };

            format!("{}{}{}", node.val, left, right)
        } else {
            "".to_string()
        }
    }
}

fn new_node(i: usize, data: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if data.len() <= i {
        None
    } else {
        let node = if let Some(v) = data[i] {
            Some(TreeNode::new(v))
        } else {
            None
        };
        let left = new_node(i + i + 1, data);
        let right = if data.len() > 1 {
            new_node(i + i + 2, data)
        } else {
            None
        };

        match node {
            Some(mut node) => {
                node.left = left;
                node.right = right;

                Some(Rc::new(RefCell::new(node)))
            }
            _ => None,
        }
    }
}

pub fn run(arg: &str) {
    println!("[p6]LeetCode 606. 根据二叉树创建字符串: {}", arg);

    assert_eq!(
        Solution::tree2str(new_node(0, &[Some(1), Some(2), Some(3), Some(4)])),
        "1(2(4))(3)"
    );

    assert_eq!(
        Solution::tree2str(new_node(0, &[Some(1), Some(2), Some(3), None, Some(4)])),
        "1(2()(4))(3)"
    );
}
