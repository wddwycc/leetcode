pub struct Solution;
impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut k = k;
        for i in 1..=(n / 2) {
            if n % i == 0 {
                k -= 1
            }
            if k == 0 {
                return i;
            }
        }
        if k == 1 {
            return n;
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let c = Solution::kth_factor(7, 2);
    }
}
