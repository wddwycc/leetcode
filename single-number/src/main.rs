pub fn single_number(nums: Vec<i32>) -> i32 {
    let sorted = {
        let mut rv = nums.clone();
        rv.sort();
        rv
    };

    for i in 0..sorted.len() {
        if i % 2 != 0 {
            continue;
        }
        if sorted.get(i) == sorted.get(i + 1) {
            continue;
        } else {
            return sorted.get(i).unwrap().clone();
        }
    }

    return 0;
}

fn main() {
    assert_eq!(single_number(vec![2, 2, 1]), 1);
}
