pub struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // give f(s) = k, represents to sum up to s, min coin nums is k
        // f(0) = 0
        // f(s) = min { for coin in coins:  f(s - coin) } + 1

        // let's try bottom-up dp
        let mut res: Vec<Option<i32>> = vec![Some(0)];
        for i in 1..=amount {
            let v = coins
                .iter()
                .filter_map(|coin| {
                    if i >= *coin {
                        res[(i - *coin) as usize]
                    } else {
                        None
                    }
                })
                .min()
                .map(|a| a + 1);
            res.push(v);
        }
        res.last().and_then(|a| *a).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }
}
