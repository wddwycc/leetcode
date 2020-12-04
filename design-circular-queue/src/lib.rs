use std::collections::VecDeque;

struct MyCircularQueue {
    capacity: usize,
    data: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    /** Initialize your data structure here. Set the size of the queue to be k. */
    fn new(k: i32) -> Self {
        Self {
            capacity: k as usize,
            data: VecDeque::with_capacity(k as usize),
        }
    }

    /** Insert an element into the circular queue. Return true if the operation is successful. */
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data.push_back(value);
        return true;
    }

    /** Delete an element from the circular queue. Return true if the operation is successful. */
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.data.pop_front();
        return true;
    }

    /** Get the front item from the queue. */
    fn front(&self) -> i32 {
        *self.data.front().unwrap_or(&-1)
    }

    /** Get the last item from the queue. */
    fn rear(&self) -> i32 {
        *self.data.back().unwrap_or(&-1)
    }

    /** Checks whether the circular queue is empty or not. */
    fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /** Checks whether the circular queue is full or not. */
    fn is_full(&self) -> bool {
        self.data.len() == self.capacity
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
