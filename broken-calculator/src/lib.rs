pub struct Solution;
impl Solution {
    pub fn broken_calc(x: i32, mut y: i32) -> i32 {
        let mut res = 0;
        while y > x {
            res += 1;
            if y % 2 == 1 {
                y += 1;
            } else {
                y /= 2;
            }
        }
        res + x - y
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
