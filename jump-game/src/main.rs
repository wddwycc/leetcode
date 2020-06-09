pub fn can_jump(nums: Vec<i32>) -> bool {
    let len = nums.len();
    if len == 0 {
        return false;
    }
    if len == 1 {
        return true;
    }

    let mut good_idx_set = std::collections::HashSet::new();
    good_idx_set.insert(len - 1);

    let mut good_idx = vec![len - 1];
    while let Some(idx) = good_idx.pop() {
        let mut j = idx - 1;
        loop {
            if (nums[j] as usize) >= idx - j {
                if j == 0 {
                    return true;
                } else {
                    if !good_idx_set.contains(&j) {
                        good_idx_set.insert(j);
                        good_idx.push(j);
                    }
                }
            }
            if j == 0 {
                break;
            } else {
                j -= 1;
            }
        }
    }
    return false;
}

fn main() {
    assert_eq!(can_jump(vec![0]), true);
    assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
}
