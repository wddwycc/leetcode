pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut last_zero_idx = None;
    for idx in 0..nums.len() {
        // when met zero
        if nums[idx] == 0 {
            match last_zero_idx {
                None => last_zero_idx = Some(idx),
                _ => (),
            }
            continue;
        }
        // when met non-zero
        match last_zero_idx {
            Some(last_zero_idx_) => {
                nums.swap(last_zero_idx_, idx);
                last_zero_idx = Some(last_zero_idx_ + 1);
            }
            None => (),
        }
    }
}

fn main() {
    let mut ary = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut ary);
    assert_eq!(ary, vec![1, 3, 12, 0, 0]);
}
