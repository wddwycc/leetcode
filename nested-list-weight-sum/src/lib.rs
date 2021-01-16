#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct Solution;
impl Solution {
    pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        let mut res = 0;
        Self::helper(nested_list, 1, &mut res);
        res
    }

    pub fn helper(src: Vec<NestedInteger>, depth: i32, res: &mut i32) {
        for item in src {
            match item {
                NestedInteger::Int(a) => *res += a * depth,
                NestedInteger::List(a) => Self::helper(a, depth + 1, res),
            }
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
