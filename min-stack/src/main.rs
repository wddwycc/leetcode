struct MinStack {
    values: Vec<i32>,
    mins: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            mins: vec![],
            values: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.values.push(x);
        match self.mins.last() {
            Some(&a) => self.mins.push(std::cmp::min(a, x)),
            None => self.mins.push(x),
        }
    }

    fn pop(&mut self) {
        self.values.pop();
        self.mins.pop();
    }

    fn top(&self) -> i32 {
        *self.values.last().unwrap()
        // self.stack.last()
    }

    fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    let a = min_stack.get_min(); // return -3
    min_stack.pop();
    let b = min_stack.top(); // return 0
    let c = min_stack.get_min(); // return -2

    println!("{}, {}, {}", a, b, c)
}
