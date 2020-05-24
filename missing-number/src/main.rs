pub fn missing_number(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let sum = (1 + len) * len / 2;
    return (sum as i32) - (nums.iter().sum::<i32>());
}

fn main() {
    assert_eq!(missing_number(vec![3, 0, 1]), 2);
}
