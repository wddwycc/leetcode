use std::{collections::VecDeque, ops::IndexMut};

#[derive(Clone, Debug)]
pub struct Trie {
    children: Vec<Option<Trie>>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            children: vec![None; 26],
        }
    }

    pub fn append(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            let idx = c as usize - 'a' as usize;
            cur = cur.children.index_mut(idx).get_or_insert(Trie::new());
        }
    }
}

pub struct Solution;
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut trie = Trie::new();
        for word in words {
            trie.append(word.chars().rev().collect());
        }
        // BFS to find all paths
        let mut res = 0;
        let mut level = 0;
        let mut queue = VecDeque::new();
        queue.push_back(trie);
        while queue.len() > 0 {
            for _ in 0..queue.len() {
                let t = queue.pop_front().unwrap();
                let children: Vec<Trie> = t.children.into_iter().filter_map(|a| a).collect();
                if children.len() == 0 {
                    res += level + 1;
                } else {
                    for child in children {
                        queue.push_back(child);
                    }
                }
            }
            level += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        trie.append("hello".to_owned());
        println!("{:?}", trie);
        assert_eq!(2 + 2, 4);
    }
}
