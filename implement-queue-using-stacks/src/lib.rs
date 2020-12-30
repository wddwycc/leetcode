#[derive(Default)]
struct MyQueue {
    pub inbox: Vec<i32>,
    pub outbox: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.inbox.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if let Some(a) = self.outbox.pop() {
            return a;
        } else {
            while let Some(a) = self.inbox.pop() {
                self.outbox.push(a)
            }
            return self.outbox.pop().unwrap_or(-1);
        }
    }

    /** Get the front element. */
    fn peek(&self) -> i32 {
        *self.outbox.last().or(self.inbox.first()).unwrap_or(&-1)
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.inbox.is_empty() && self.outbox.is_empty()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
