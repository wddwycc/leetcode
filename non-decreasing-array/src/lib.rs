pub struct Solution {}
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let len = nums.len();
        if len <= 1 {
            return true;
        }
        let mut updates = 0;
        for cur in 0..(len - 1) {
            if &nums[cur] <= &nums[cur + 1] {
                continue;
            }
            updates += 1;
            if cur == 0 {
                nums[cur] = nums[cur + 1]
            } else if cur == len - 2 {
                nums[cur + 1] = nums[cur]
            } else {
                let mut i = cur as isize - 1;
                let mut i_updates = 0;
                while i >= 0 {
                    if nums[i as usize] <= nums[cur + 1] {
                        break;
                    } else {
                        i_updates += 1;
                        i -= 1
                    }
                }
                if i_updates == 0 {
                    nums[cur] = nums[cur + 1];
                }

                let mut j = cur + 2;
                let mut j_updates = 0;
                while j <= len - 1 {
                    if nums[j] >= nums[cur] {
                        break;
                    } else {
                        j_updates += 1;
                        j += 1;
                    }
                }
                if i_updates == 0 {
                    nums[cur] = nums[cur + 1];
                } else if j_updates == 0 {
                    nums[cur + 1] = nums[cur];
                } else {
                    return false;
                }
            }
            if updates > 1 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
        assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
        assert_eq!(Solution::check_possibility(vec![3, 4, 2, 3]), false);
        assert_eq!(Solution::check_possibility(vec![5, 7, 1, 8]), true);
    }
}
