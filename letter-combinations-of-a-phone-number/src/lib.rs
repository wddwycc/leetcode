pub struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = vec![];
        for digit in digits.chars() {
            let chars = match digit {
                '2' => vec!['a', 'b', 'c'],
                '3' => vec!['d', 'e', 'f'],
                '4' => vec!['g', 'h', 'i'],
                '5' => vec!['j', 'k', 'l'],
                '6' => vec!['m', 'n', 'o'],
                '7' => vec!['p', 'q', 'r', 's'],
                '8' => vec!['t', 'u', 'v'],
                '9' => vec!['w', 'x', 'y', 'z'],
                _ => continue,
            };
            if res.len() == 0 {
                res = chars.into_iter().map(|a| a.to_string()).collect();
            } else {
                res = res
                    .iter()
                    .flat_map(|a| {
                        chars
                            .iter()
                            .map(|b| {
                                let mut res = a.clone();
                                res.push(b.clone());
                                res
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect()
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
