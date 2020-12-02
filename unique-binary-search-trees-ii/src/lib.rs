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
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;
impl Solution {
    fn helper(l: i32, r: i32) -> Vec<Rc<RefCell<TreeNode>>> {
        match r - l {
            0 => vec![Rc::new(RefCell::new(TreeNode::new(l)))],
            1 => vec![
                Rc::new(RefCell::new(TreeNode {
                    val: l,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(r)))),
                })),
                Rc::new(RefCell::new(TreeNode {
                    val: r,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(l)))),
                    right: None,
                })),
            ],
            _ => {
                let mut res = vec![];
                Self::helper(l + 1, r)
                    .into_iter()
                    .map(|node| {
                        Rc::new(RefCell::new(TreeNode {
                            val: l,
                            left: None,
                            right: Some(node),
                        }))
                    })
                    .for_each(|a| res.push(a));
                for pivot in (l + 1)..=(r - 1) {
                    for i in Self::helper(l, pivot - 1).iter() {
                        for j in Self::helper(pivot + 1, r).iter() {
                            res.push(Rc::new(RefCell::new(TreeNode {
                                val: pivot,
                                left: Some(i.clone()),
                                right: Some(j.clone()),
                            })))
                        }
                    }
                }
                Self::helper(l, r - 1)
                    .into_iter()
                    .map(|node| {
                        Rc::new(RefCell::new(TreeNode {
                            val: r,
                            left: Some(node),
                            right: None,
                        }))
                    })
                    .for_each(|a| res.push(a));
                res
            }
        }
    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            vec![]
        } else {
            Self::helper(1, n).into_iter().map(|a| Some(a)).collect()
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
