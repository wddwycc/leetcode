fn main() {
    println!("Hello, world!");
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }
    let mut i = 0;
    let mut j = 0;
    while j < len {
        // when duplicate

        if nums[j] == nums[i] {
            j += 1;
            continue;
        }

        // when not duplicate

        if j == i + 1 {
            // when sibling
            i += 1;
            j += 1;
            continue;
        } else {
            // when not sibling
            i += 1;
            nums[i] = nums[j];
            j += 1;
        }
    }
    (i + 1) as i32
}
