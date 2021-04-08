pub struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let digits = digits.as_bytes();
        let mut res = vec![];
        Self::dfs(digits, 0, &mut vec![], &mut res);
        res
    }

    fn dfs(digits: &[u8], cur: usize, acc: &mut Vec<u8>, res: &mut Vec<String>) {
        if cur == digits.len() {
            res.push(String::from_utf8(acc.clone()).unwrap());
            return;
        }
        let letters = match digits[cur] {
            b'2' => [b'a', b'b', b'c'].iter(),
            b'3' => [b'd', b'e', b'f'].iter(),
            b'4' => [b'g', b'h', b'i'].iter(),
            b'5' => [b'j', b'k', b'l'].iter(),
            b'6' => [b'm', b'n', b'o'].iter(),
            b'7' => [b'p', b'q', b'r', b's'].iter(),
            b'8' => [b't', b'u', b'v'].iter(),
            b'9' => [b'w', b'x', b'y', b'z'].iter(),
            _ => panic!(),
        };
        for &letter in letters {
            acc.push(letter);
            Self::dfs(digits, cur + 1, acc, res);
            acc.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
