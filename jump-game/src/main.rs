pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut step = vec![0; nums.len()];
    for i in 1..nums.len() {
        step[i] = step[i - 1].max(nums[i - 1]) - 1;
        if step[i] < 0 {
            return false;
        }
    }
    true
}

fn main() {
    assert_eq!(can_jump(vec![0]), true);
    assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
}
