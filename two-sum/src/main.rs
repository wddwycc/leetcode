pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut rests = std::collections::HashMap::new();
    for idx in 0..nums.len() {
        let rest = target - nums[idx];
        match rests.get(&rest) {
            Some(&prev_idx) => return vec![prev_idx, idx as i32],
            None => {
                rests.insert(nums[idx], idx as i32);
            }
        }
    }
    return vec![];
}

fn main() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
