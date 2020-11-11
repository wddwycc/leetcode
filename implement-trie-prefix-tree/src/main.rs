use std::collections::HashMap;

struct Trie {
    data: HashMap<char, Trie>,
    is_word: bool
}

impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            is_word: false
        }
    }
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.data.entry(c).or_insert(Trie::new());
        }
        cur.is_word = true;
    }
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for c in word.chars() {
            match cur.data.get(&c) {
                Some(a) => cur = a,
                None => return false
            }
        }
        cur.is_word
    }
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for c in prefix.chars() {
            match cur.data.get(&c) {
                Some(a) => cur = a,
                None => return false
            }
        }
        cur.is_word || cur.data.len() > 0 
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
}
