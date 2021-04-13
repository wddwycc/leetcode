// #[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl NestedInteger {
    fn flatten(self) -> Vec<i32> {
        match self {
            Self::Int(a) => vec![a],
            Self::List(list) => list.into_iter().map(|a| a.flatten()).flatten().collect(),
        }
    }
}

struct NestedIterator {
    cur: usize,
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        Self {
            cur: 0,
            data: NestedInteger::List(nestedList).flatten(),
        }
    }

    fn next(&mut self) -> i32 {
        if self.has_next() {
            let data = self.data[self.cur];
            self.cur += 1;
            return data;
        } else {
            return -1;
        }
    }

    fn has_next(&self) -> bool {
        self.cur < self.data.len()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
