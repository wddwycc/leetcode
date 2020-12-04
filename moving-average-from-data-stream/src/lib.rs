use std::collections::VecDeque;

struct MovingAverage {
    capacity: usize,
    deque: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        Self {
            capacity: size as usize,
            deque: VecDeque::with_capacity(size as usize),
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.deque.len() == self.capacity {
            self.deque.pop_front();
        }
        self.deque.push_back(val);
        self.deque.iter().sum::<i32>() as f64 / self.deque.len() as f64
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
