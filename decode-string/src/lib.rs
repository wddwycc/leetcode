pub struct Solution;
impl Solution {
    pub fn decode_string(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let (_, res) = Self::decode(&chars, 0);
        res.iter().collect::<String>()
    }

    pub fn num_from_digits(src: Vec<u8>) -> usize {
        let len = src.len();
        let mut exponent = src.len() - 1;
        let mut res: usize = 0;
        loop {
            res += src[len - exponent - 1] as usize * (10 as usize).pow(exponent as u32);
            if exponent == 0 {
                break;
            } else {
                exponent -= 1;
            }
        }
        res
    }

    /// returns (end_cur, data)
    pub fn decode(src: &[char], mut cur: usize) -> (usize, Vec<char>) {
        let mut res = vec![];
        // NOTE: met ] or reach end, return res
        while cur < src.len() && src[cur] != ']' {
            let char = src[cur];
            // NOTE: is char
            if char.is_alphabetic() {
                res.push(char);
                cur += 1;
                continue;
            }
            // NOTE: is digits[xx];
            // NOTE: retrieve digits
            let mut digits = vec![];
            loop {
                match src[cur].to_string().parse::<u8>() {
                    Ok(a) => digits.push(a),
                    Err(_) => break,
                };
                cur += 1;
            }
            let num = Self::num_from_digits(digits);
            let (end_cur, data) = Self::decode(src, cur + 1);
            for _ in 0..num {
                res.append(&mut data.clone());
            }
            cur = end_cur + 1;
        }
        (cur, res)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
