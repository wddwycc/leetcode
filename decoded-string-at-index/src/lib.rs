enum Entity {
    Chars(Vec<char>),
    Repeat(usize),
}

pub struct Solution;
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        // NOTE: Parse input
        let seq = {
            let mut res = vec![];
            let mut tmp = vec![];
            for c in s.chars() {
                if c.is_alphabetic() {
                    tmp.push(c);
                } else {
                    res.push(Entity::Chars(tmp));
                    tmp = vec![];
                    let times = c.to_string().parse::<usize>().unwrap();
                    res.push(Entity::Repeat(times));
                }
            }
            if !tmp.is_empty() {
                res.push(Entity::Chars(tmp))
            }
            res
        };
        Self::char_at(&seq, k as usize - 1).to_string()
    }

    fn char_at(entities: &[Entity], target: usize) -> char {
        let mut len = 0;
        for entity in entities {
            match entity {
                Entity::Chars(a) => {
                    if target < len + a.len() {
                        return a[target - len];
                    } else {
                        len += a.len();
                    }
                }
                Entity::Repeat(n) => {
                    if target < len * n {
                        return Self::char_at(entities, target % len);
                    } else {
                        len *= n
                    }
                }
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
