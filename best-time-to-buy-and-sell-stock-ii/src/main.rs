pub fn max_profit(prices: Vec<i32>) -> i32 {
    let len = prices.len();
    if len < 2 {
        return 0;
    }

    let mut profits = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            profits += prices[i] - prices[i - 1];
        }
    }
    profits
}

fn main() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
}
