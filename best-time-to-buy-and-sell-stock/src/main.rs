use std::cmp::{max, min};

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    if len < 2 {
        return 0;
    }

    let mut min_price = prices[0];
    let mut max_profit = 0;

    for i in 1..len {
        min_price = min(min_price, prices[i]);
        max_profit = max(max_profit, prices[i] - min_price)
    }

    max_profit
}

fn main() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}
