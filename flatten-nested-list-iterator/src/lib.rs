#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    stack: Vec<NestedInteger>,
}

impl NestedIterator {
    fn new(mut nestedList: Vec<NestedInteger>) -> Self {
        nestedList.reverse();
        Self { stack: nestedList }
    }

    fn consume(&mut self) -> Option<i32> {
        if let Some(nested_i) = self.stack.pop() {
            match nested_i {
                NestedInteger::Int(a) => return Some(a),
                NestedInteger::List(mut list) => {
                    list.reverse();
                    for a in list {
                        self.stack.push(a);
                    }
                    return self.consume();
                }
            }
        }
        None
    }

    fn next(&mut self) -> i32 {
        self.consume().unwrap_or(-1)
    }

    fn has_next(&mut self) -> bool {
        if let Some(a) = self.consume() {
            self.stack.push(NestedInteger::Int(a));
            return true;
        } else {
            return false;
        }
    }
}
