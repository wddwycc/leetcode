pub fn reverse(x: i32) -> i32 {
    let is_negative = x < 0;
    let mut x = x.abs();
    let mut nums = Vec::new();
    while x > 0 {
        nums.push(x % 10);
        x = x / 10;
    }

    let mut rv = 0;
    let len = nums.len();
    let mut n = len;
    while n > 0 {
        match nums[len - n]
            .checked_mul(i32::pow(10, (n as u32) - 1))
            .and_then(|a| a.checked_add(rv))
        {
            Some(a) => {
                rv = a;
                n -= 1;
            }
            None => {
                return 0;
            }
        }
    }
    if is_negative {
        return -rv;
    }
    return rv;
}

fn main() {
    assert_eq!(reverse(1534236469), 0);
    assert_eq!(reverse(1234), 4321);
}
