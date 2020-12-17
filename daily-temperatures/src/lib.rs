pub struct Solution;
impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0; t.len()];
        let mut stack: Vec<usize> = Vec::new();
        for (i, &temp) in t.iter().enumerate().rev() {
            while let Some(&j) = stack.last() {
                if temp >= t[j] {
                    stack.pop();
                } else {
                    break;
                }
            }

            res[i] = match stack.last() {
                Some(e) => (e - i) as i32,
                None => 0,
            };
            stack.push(i);
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
