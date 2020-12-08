pub struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn encode(&self, strs: Vec<String>) -> String {
        let mut res: String = String::from("");
        for s in strs {
            let chunk_header = std::char::from_u32(s.len() as u32).unwrap();
            res.push(chunk_header);
            res.push_str(&s);
        }
        res
    }

    pub fn decode(&self, s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();

        let mut res = vec![];
        let mut cur = 0;
        while cur < chars.len() {
            let len = chars[cur] as usize;
            let chunk = chars[(cur + 1)..=(cur + len)].iter().collect();
            res.push(chunk);
            cur += 1 + len;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(std::char::from_u32('💩' as u32), Some('💩'));

        let codec = Codec::new();
        assert_eq!(
            codec.decode(codec.encode(vec!["".to_string(), " ".to_string(), "world".to_string()])),
            vec!["".to_string(), " ".to_string(), "world".to_string()],
        );
    }
}
