pub struct Solution;
impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        // if we know distributive property of bitwise operator:
        // (a1^a2) & (b1^b2) = (a1&b1) ^ (a1&b2) ^ (a2&b1) ^ (a2&b2)
        arr1.into_iter().fold(0, |a, b| a ^ b) & arr2.into_iter().fold(0, |a, b| a ^ b)
    }
}
