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
use std::cell::RefCell;
use std::rc::Rc;

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = vec![];
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            let node = match node {
                Some(a) => a,
                None => {
                    res.push("null".to_string());
                    continue;
                }
            };
            res.push(node.borrow().val.to_string());
            stack.push(node.borrow().right.clone());
            stack.push(node.borrow().left.clone());
        }
        res.join(",")
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let lst: Vec<Option<i32>> = data
            .split(",")
            .map(|s| {
                if s == "null" {
                    None
                } else {
                    Some(s.parse::<i32>().unwrap())
                }
            })
            .collect();
        let root = match lst[0] {
            Some(a) => Rc::new(RefCell::new(TreeNode::new(a))),
            None => return None,
        };
        let mut cur = 1;
        let mut stack = vec![(root.clone(), false)];
        while let Some((node, left_seen)) = stack.pop() {
            if left_seen {
                if let Some(v) = lst[cur] {
                    let r = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(r.clone());
                    stack.push((r.clone(), false));
                }
            } else {
                stack.push((node.clone(), true));
                if let Some(v) = lst[cur] {
                    let l = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(l.clone());
                    stack.push((l.clone(), false));
                }
            }
            cur += 1;
        }
        Some(root)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
