use std::cmp::Ordering;

pub struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;
        if x < 2 {
            return x as i32;
        }
        let mut l = 2;
        let mut r = x / 2;
        while l <= r {
            let mid = l + (r - l) / 2;
            match (mid * mid).cmp(&x) {
                Ordering::Greater => r = mid - 1,
                Ordering::Less => l = mid + 1,
                Ordering::Equal => return mid as i32,
            };
        }
        return r as i32;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
