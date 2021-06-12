struct NumArray {
    prefix_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        Self { prefix_sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix_sum[right as usize + 1] - self.prefix_sum[left as usize]
    }
}
