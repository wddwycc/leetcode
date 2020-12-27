use std::collections::HashMap;

pub struct Solution;
impl Solution {
    fn calc(src: &[char], cur: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if let Some(cached) = cache.get(&cur) {
            return *cached;
        }
        let res = {
            if cur >= src.len() {
                1
            } else {
                match src[cur] {
                    '0' => 0,
                    '1' => {
                        // if has next,
                        //   if next is 0, force two digits decoder
                        //   else one | two digits decoder
                        // else, return 1
                        if cur + 1 < src.len() {
                            if src[cur + 1] == '0' {
                                Self::calc(src, cur + 2, cache)
                            } else {
                                Self::calc(src, cur + 1, cache) + Self::calc(src, cur + 2, cache)
                            }
                        } else {
                            1
                        }
                    }
                    '2' => {
                        // if has next,
                        //   if next is 0, force two digits decoder
                        //   else if next is 7 | 8 | 9, force one
                        //   else one | two digits decoder
                        // else, return times
                        if cur + 1 < src.len() {
                            match src[cur + 1] {
                                '0' => Self::calc(src, cur + 2, cache),
                                '7' | '8' | '9' => Self::calc(src, cur + 1, cache),
                                _ => {
                                    Self::calc(src, cur + 1, cache)
                                        + Self::calc(src, cur + 2, cache)
                                }
                            }
                        } else {
                            1
                        }
                    }
                    _ => Self::calc(src, cur + 1, cache),
                }
            }
        };
        cache.insert(cur, res);
        res
    }

    pub fn num_decodings(s: String) -> i32 {
        let mut cache = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        Self::calc(&chars, 0, &mut cache) as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
