#[derive(Default)]
pub struct Trie {
    pub children: [Option<Box<Trie>>; 26],
    pub is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            let idx = (c as i8 - 'a' as i8) as usize;
            cur = cur.children[idx].get_or_insert_with(|| Box::new(Trie::new()));
        }
        cur.is_word = true;
    }

    pub fn get_root(&self, word: &str) -> Option<String> {
        let mut cur = self;
        let mut chars: Vec<char> = vec![];
        for c in word.chars() {
            let idx = (c as i8 - 'a' as i8) as usize;
            match cur.children[idx].as_ref() {
                Some(a) => cur = a,
                None => return None,
            }
            chars.push(c);
            if cur.is_word {
                return Some(chars.into_iter().collect());
            }
        }
        return None;
    }
}

pub struct Solution {}
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut trie = Trie::new();
        for word in dictionary {
            trie.insert(word)
        }
        sentence
            .split(' ')
            .map(|w| trie.get_root(w).unwrap_or(w.to_string()))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let res = Solution::replace_words(
            vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
            "the cattle was rattled by the battery".to_string(),
        );
        assert_eq!(res, "the cat was rat by the bat".to_string());
    }
}
