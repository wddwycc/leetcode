pub struct Solution;
impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        // if we know distributive property of bitwise:
        // (a1^a2) & (b1^b2) = (a1&b1) ^ (a1&b2) ^ (a2&b1) ^ (a2&b2)
        let l = arr1.into_iter().fold(0, |res, v| res ^ v);
        let r = arr2.into_iter().fold(0, |res, v| res ^ v);
        l & r
    }
}
