pub struct Solution;
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut dists = vec![];
        for i in 0..32 {
            let mut dist = vec![0; 10];
            for c in (2 as i64).pow(i as u32).to_string().chars() {
                dist[c.to_digit(10).unwrap() as usize] += 1;
            }
            dists.push(dist)
        }
        let target_dist = {
            let mut dist = vec![0; 10];
            for c in n.to_string().chars() {
                dist[c.to_digit(10).unwrap() as usize] += 1;
            }
            dist
        };
        dists.contains(&target_dist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reordered_power_of2(1), true)
    }
}
