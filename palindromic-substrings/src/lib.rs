pub struct Solution;
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut res = 0;
        let bytes = s.as_bytes();
        for i in 0..(bytes.len() as i32) {
            res += Self::count_for_center(bytes, i, i);
            res += Self::count_for_center(bytes, i, i + 1);
        }
        res
    }

    fn count_for_center(bytes: &[u8], mut i: i32, mut j: i32) -> i32 {
        let mut res = 0;
        while i >= 0 && j < bytes.len() as i32 {
            if bytes[i as usize] == bytes[j as usize] {
                res += 1;
            } else {
                break;
            }
            i -= 1;
            j += 1;
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
