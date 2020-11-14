#[derive(Default)]
struct Trie {
    data: [Option<Box<Trie>>; 26],
    is_word: bool
}

impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            let idx = (c as i8 - 'a' as i8) as usize;
            cur = cur.data[idx].get_or_insert_with(|| Box::new(Trie::new()));
        }
        cur.is_word = true;
    }
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for c in word.chars() {
            let idx = (c as i8 - 'a' as i8) as usize;
            match cur.data[idx].as_ref() {
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
            let idx = (c as i8 - 'a' as i8) as usize;
            match &cur.data[idx] {
                Some(a) => cur = a,
                None => return false
            }
        }
        true
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
