use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

struct Solution {
    origin: Vec<i32>,
    shuffled: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            origin: nums.clone(),
            shuffled: nums,
            rng: thread_rng(),
        }
    }
    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.origin.clone()
    }
    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        self.shuffled.shuffle(&mut self.rng);
        self.shuffled.clone()
    }
}

fn main() {
    let mut obj = Solution::new(vec![1, 2, 3]);
    let ret_1: Vec<i32> = obj.reset();
    let ret_2: Vec<i32> = obj.shuffle();
}
