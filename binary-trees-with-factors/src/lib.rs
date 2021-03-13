use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.sort();

        let mut products: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        let mut dist = HashMap::new();

        for i in 0..arr.len() {
            let mut count = 1;
            if let Some(pairs) = products.get(&arr[i]) {
                count += pairs
                    .iter()
                    .map(|(l, r)| {
                        let mut res = dist.get(l).unwrap() * dist.get(r).unwrap();
                        if l != r {
                            res *= 2;
                        }
                        res
                    })
                    .sum::<i32>();
            }
            for j in 0..=i {
                if let Some(k) = arr[i].checked_mul(arr[j]) {
                    products.entry(k).or_insert(vec![]).push((arr[i], arr[j]));
                }
            }
            dist.insert(arr[i], count);
        }
        println!("{:?}", dist);
        dist.values().sum()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
