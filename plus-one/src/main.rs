pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let len = digits.len();
    if len < 1 {
        return vec![];
    }

    let mut cur = len - 1;
    while digits[cur] == 9 {
        digits[cur] = 0;
        if cur == 0 {
            digits.insert(0, 1);
            return digits;
        } else {
            cur -= 1;
        }
    }
    digits[cur] += 1;
    digits
}

fn main() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![9, 9]), vec![1, 0, 0]);
}
