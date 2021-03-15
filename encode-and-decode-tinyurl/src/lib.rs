use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[derive(Default)]
struct Codec {
    dict: RefCell<HashMap<String, String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Default::default()
    }

    // Encodes a URL to a shortened URL.
    fn encode(&self, longURL: String) -> String {
        let hex_digest = {
            let mut s = DefaultHasher::new();
            longURL.hash(&mut s);
            let s = s.finish();
            format!("{:x}", s)
        };
        self.dict.borrow_mut().insert(hex_digest.clone(), longURL);
        format!("http://tinyurl.com/{}", hex_digest)
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        let hex_digest = shortURL.replace("http://tinyurl.com/", "");
        if let Some(a) = self.dict.borrow().get(&hex_digest) {
            return a.clone();
        }
        return "".to_owned();
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
