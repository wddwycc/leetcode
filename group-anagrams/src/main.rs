use std::collections::HashMap;
use std::iter::FromIterator;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut group: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        let sorted = String::from_iter(chars);
        group.entry(sorted).or_insert(vec![]).push(s);
    }
    let mut res = vec![];
    for (_, v) in group.into_iter() {
        res.push(v)
    }
    res
}

fn main() {
    println!("Hello, world!");
}
