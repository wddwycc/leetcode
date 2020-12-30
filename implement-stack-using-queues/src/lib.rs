use std::collections::VecDeque;

#[derive(Default)]
struct MyStack {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.queue.len() == 0 {
            return -1;
        }
        let mut next_queue = VecDeque::new();
        for _ in 0..(self.queue.len() - 1) {
            let a = self.queue.pop_front().unwrap();
            next_queue.push_back(a);
        }
        let res = self.queue.pop_front().unwrap();
        self.queue = next_queue;
        return res;
    }

    /** Get the top element. */
    fn top(&mut self) -> i32 {
        if self.queue.len() == 0 {
            return -1;
        }
        let mut next_queue = VecDeque::new();
        for _ in 0..(self.queue.len() - 1) {
            let a = self.queue.pop_front().unwrap();
            next_queue.push_back(a);
        }
        let res = *self.queue.front().unwrap();
        next_queue.push_back(self.queue.pop_front().unwrap());
        self.queue = next_queue;
        return res;
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
