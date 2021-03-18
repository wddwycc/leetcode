use std::cmp::Ordering;

enum State {
    Pending(i32),      // prev
    NeedIncrease(i32), // min
    NeedDecrease(i32), // max
}

pub struct Solution;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut state = State::Pending(nums[0]);

        for i in 1..nums.len() {
            let num = nums[i];
            state = match state {
                State::Pending(prev) => match num.cmp(&prev) {
                    Ordering::Equal => State::Pending(prev),
                    Ordering::Greater => {
                        res += 1;
                        State::NeedDecrease(num)
                    }
                    Ordering::Less => {
                        res += 1;
                        State::NeedIncrease(num)
                    }
                },
                State::NeedIncrease(min) => match num.cmp(&min) {
                    Ordering::Greater => {
                        res += 1;
                        State::NeedDecrease(num)
                    }
                    _ => State::NeedIncrease(num),
                },
                State::NeedDecrease(max) => match num.cmp(&max) {
                    Ordering::Less => {
                        res += 1;
                        State::NeedIncrease(num)
                    }
                    _ => State::NeedDecrease(num),
                },
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
