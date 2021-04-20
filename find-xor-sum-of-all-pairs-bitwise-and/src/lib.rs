pub struct Solution;
impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        // use arr1[0] to pair with all arr2, every i-th bit can be calculated separately:
        // a1&b1 ^ a1&b2 ^ a1&b3 .... ^ a1&bn
        // if a1 == 0, ans = 0
        // if a1 == 1, ans = b1 ^ b2 ... ^ bn, result depends on how many 1s ranged from { b1...bn }
        let mut ones = vec![0; 31];
        for b in arr2 {
            for i in 0..31 {
                if (b >> i) & 1 == 1 {
                    ones[i] += 1;
                }
            }
        }
        let mut ans = 0;
        for a in arr1 {
            let mut res = 0;
            for i in 0..31 {
                if ((a >> i) & 1) == 1 && ones[i] % 2 == 1 {
                    res += 1 << i
                }
            }
            ans ^= res;
        }
        ans
    }
}
