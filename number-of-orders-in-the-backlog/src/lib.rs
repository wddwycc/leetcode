use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

#[derive(Debug, PartialEq, Eq)]
struct Orders {
    price: i32,
    amount: i32,
}

impl PartialOrd for Orders {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Orders {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

pub struct Solution;
impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        let mut buy_heap: BinaryHeap<Orders> = BinaryHeap::new();
        let mut sell_heap: BinaryHeap<Reverse<Orders>> = BinaryHeap::new(); // Reverse<i32>

        for order in orders {
            let price = order[0];
            let mut amount = order[1];
            let is_buy = order[2] == 0;
            if is_buy {
                loop {
                    if let Some(sell_orders) = sell_heap.pop() {
                        if price >= sell_orders.0.price {
                            if amount >= sell_orders.0.amount {
                                amount -= sell_orders.0.amount;
                            } else {
                                sell_heap.push(Reverse(Orders {
                                    price: sell_orders.0.price,
                                    amount: sell_orders.0.amount - amount,
                                }));
                                amount = 0;
                                break;
                            }
                        } else {
                            sell_heap.push(sell_orders);
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if amount > 0 {
                    buy_heap.push(Orders { price, amount });
                }
            } else {
                loop {
                    if let Some(buy_orders) = buy_heap.pop() {
                        if price <= buy_orders.price {
                            if amount >= buy_orders.amount {
                                amount -= buy_orders.amount;
                            } else {
                                buy_heap.push(Orders {
                                    price: buy_orders.price,
                                    amount: buy_orders.amount - amount,
                                });
                                amount = 0;
                                break;
                            }
                        } else {
                            buy_heap.push(buy_orders);
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if amount > 0 {
                    sell_heap.push(Reverse(Orders { price, amount }));
                }
            }
            println!("round:");
            println!("buy: {:?}", buy_heap);
            println!("sell: {:?}", sell_heap);
        }
        let modolo = (10 as i64).pow(9) + 7;
        let res = buy_heap.iter().map(|a| a.amount as i64).sum::<i64>()
            + sell_heap.iter().map(|a| a.0.amount as i64).sum::<i64>();
        (res % modolo) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let inputs = vec![
            vec![10, 5, 0],
            vec![15, 2, 1],
            vec![25, 1, 1],
            vec![30, 4, 0],
        ];
        let res = Solution::get_number_of_backlog_orders(inputs);
        assert_eq!(res, 6);
    }
}
