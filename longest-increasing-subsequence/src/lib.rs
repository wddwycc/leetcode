pub struct Solution;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut piles: Vec<i32> = vec![];
        for num in nums {
            if let Err(i) = piles.binary_search(&num) {
                if i < piles.len() {
                    piles[i] = num;
                } else {
                    piles.push(num);
                }
            }
        }
        piles.len() as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
