pub struct Solution;
impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        if n <= 2 {
            return 1;
        }
        let mut res = vec![];
        res.push(0);
        res.push(1);
        let mut max = 1;
        for i in 2..=(n as usize) {
            let v = {
                if i % 2 == 0 {
                    res[i / 2]
                } else {
                    res[i / 2] + res[i / 2 + 1]
                }
            };
            max = std::cmp::max(max, v);
            res.push(v);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
