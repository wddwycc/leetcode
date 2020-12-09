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

    fn build_tree(src: &[Option<i32>], cur: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if *cur >= src.len() {
            return None;
        }
        if let Some(val) = src[*cur] {
            *cur += 1;
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                left: Self::build_tree(src, cur),
                right: Self::build_tree(src, cur),
            })))
        } else {
            *cur += 1;
            None
        }
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
        let mut cur = 0;
        Self::build_tree(&lst, &mut cur)
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
