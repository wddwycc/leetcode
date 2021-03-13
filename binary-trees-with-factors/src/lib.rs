use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut arr: Vec<i64> = arr.into_iter().map(|a| a as i64).collect();
        arr.sort();

        let mut products: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
        let mut dist = HashMap::new();
        let modulo = 10_i64.pow(9 as u32) + 7;

        for i in 0..arr.len() {
            let mut count = 1;
            if let Some(pairs) = products.get(&arr[i]) {
                for pair in pairs {
                    let (l, r) = pair;
                    let mut res = dist.get(l).unwrap() * dist.get(r).unwrap();
                    if l != r {
                        res *= 2;
                    }
                    count += res;
                }
            }
            for j in 0..=i {
                if let Some(k) = arr[i].checked_mul(arr[j]) {
                    products.entry(k).or_insert(vec![]).push((arr[i], arr[j]));
                }
            }
            dist.insert(arr[i], count);
        }

        (dist.values().sum::<i64>() % modulo) as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
