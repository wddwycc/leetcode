pub struct Solution;
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut tmp = vec![];
        let target_idx = k as usize - 1;
        for c in s.chars() {
            if c.is_alphabetic() {
                tmp.push(c);
            } else {
                let times = c.to_string().parse::<usize>().unwrap();
                tmp = tmp.repeat(times);
            }
            if target_idx < tmp.len() {
                return tmp[target_idx].to_string();
            }
        }
        panic!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
