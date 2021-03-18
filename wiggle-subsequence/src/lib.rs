use std::cmp::Ordering;

enum State {
    Pending(i32),      // prev
    NeedIncrease(i32), // min
    NeedDecrease(i32), // max
}

pub struct Solution;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        Self::helper(State::Pending(nums[0]), &nums[1..], 1)
    }

    fn helper(state: State, nums: &[i32], acc: i32) -> i32 {
        let num = match nums.first() {
            Some(a) => *a,
            None => return acc,
        };

        match state {
            State::Pending(prev) => match num.cmp(&prev) {
                Ordering::Equal => Self::helper(State::Pending(prev), &nums[1..], acc),
                Ordering::Greater => Self::helper(State::NeedDecrease(num), &nums[1..], acc + 1),
                Ordering::Less => Self::helper(State::NeedIncrease(num), &nums[1..], acc + 1),
            },
            State::NeedIncrease(min) => match num.cmp(&min) {
                Ordering::Greater => Self::helper(State::NeedDecrease(num), &nums[1..], acc + 1),
                _ => Self::helper(State::NeedIncrease(num), &nums[1..], acc),
            },
            State::NeedDecrease(max) => match num.cmp(&max) {
                Ordering::Less => Self::helper(State::NeedIncrease(num), &nums[1..], acc + 1),
                _ => return Self::helper(State::NeedDecrease(num), &nums[1..], acc),
            },
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
