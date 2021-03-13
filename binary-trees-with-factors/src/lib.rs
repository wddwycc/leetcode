use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.sort();
        let arr: Vec<i64> = arr.into_iter().map(|a| a as i64).collect();
        let max = *arr.last().unwrap();

        // product's corresponding pairs
        let mut product_pairs: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
        // HashMap<product, tree_num>
        let mut nums = HashMap::new();

        for (i, x) in arr.iter().enumerate() {
            let mut count = 1;
            if let Some(pairs) = product_pairs.get(x) {
                for pair in pairs {
                    let (l, r) = pair;
                    let mut res = nums.get(l).unwrap() * nums.get(r).unwrap();
                    if l != r {
                        res *= 2;
                    }
                    count += res;
                }
            }
            for y in arr[0..=i].iter() {
                let product = x * y;
                if product > max {
                    break;
                }
                product_pairs
                    .entry(product)
                    .or_insert(vec![])
                    .push((*x, *y));
            }
            nums.insert(x, count);
        }

        let modulo = 10_i64.pow(9 as u32) + 7;
        (nums.values().sum::<i64>() % modulo) as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
