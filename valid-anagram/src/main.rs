use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let dist1 = s.chars().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        return acc;
    });
    let dist2 = t.chars().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        return acc;
    });
    dist1 == dist2
}

fn main() {
    assert_eq!(
        is_anagram(String::from("anagram"), String::from("nagaram")),
        true
    );
}
