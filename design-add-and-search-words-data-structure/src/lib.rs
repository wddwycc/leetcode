#[derive(Default)]
pub struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    is_word: bool,
}

impl WordDictionary {
    pub fn new() -> Self {
        Default::default()
    }

    /** Adds a word into the data structure. */
    pub fn add_word(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            let idx = (c as i8 - 'a' as i8) as usize;
            cur = cur.children[idx].get_or_insert_with(|| Box::new(WordDictionary::new()));
        }
        cur.is_word = true;
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    pub fn search(&self, word: String) -> bool {
        let mut cur = self;
        for (idx, c) in word.chars().enumerate() {
            if c == '.' {
                if idx == word.len() - 1 {
                    return cur
                        .children
                        .iter()
                        .filter(|a| a.as_ref().map(|b| b.is_word).unwrap_or(false))
                        .count()
                        > 0;
                } else {
                    for child in cur.children.iter().filter_map(|a| a.as_ref()) {
                        if child.search(word[(idx + 1)..].to_string()) {
                            return true;
                        }
                    }
                    return false;
                }
            } else {
                let idx = (c as i8 - 'a' as i8) as usize;
                match cur.children[idx].as_ref() {
                    Some(a) => cur = a,
                    None => return false,
                }
            }
        }
        cur.is_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let mut wd = WordDictionary::new();
        wd.add_word("bad".to_string());
        wd.add_word("dad".to_string());
        wd.add_word("mad".to_string());

        assert_eq!(wd.search("pad".to_string()), false);
        assert_eq!(wd.search("bad".to_string()), true);
        assert_eq!(wd.search(".ad".to_string()), true);
        assert_eq!(wd.search("b..".to_string()), true);
    }

    #[test]
    fn example2() {
        let mut wd = WordDictionary::new();
        wd.add_word("a".to_string());
        wd.add_word("a".to_string());

        assert_eq!(wd.search(".".to_string()), true);
        assert_eq!(wd.search("a".to_string()), true);
        assert_eq!(wd.search("aa".to_string()), false);
        assert_eq!(wd.search("a".to_string()), true);
        assert_eq!(wd.search(".a".to_string()), false);
        assert_eq!(wd.search("a.".to_string()), false);
    }
}
