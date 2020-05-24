pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 1 {
        return nums[0];
    }

    let mut max_sum = Vec::with_capacity(len);
    max_sum.push(nums[0]);
    for i in 1..len {
        let next = std::cmp::max(nums[i] + max_sum[i - 1], nums[i]);
        max_sum.push(next);
    }

    max_sum.into_iter().max().unwrap()
}

fn main() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}
