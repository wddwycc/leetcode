/// Input: nums = [1,2,3,4,5,6,7], k = 3
/// Output: [5,6,7,1,2,3,4]
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k = (k as usize) % nums.len();
    if len == 1 || k == 0 {
        return;
    }

    reverse(nums, 0, len - 1);
    reverse(nums, 0, k - 1);
    reverse(nums, k, len - 1);
}

fn reverse(nums: &mut Vec<i32>, start: usize, end: usize) {
    let mut start = start;
    let mut end = end;
    while start < end {
        let tmp = nums[start];
        nums[start] = nums[end];
        nums[end] = tmp;
        start += 1;
        end -= 1;
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
}
