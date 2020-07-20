use std::collections::HashMap;

pub fn word_pattern(pattern: String, str: String) -> bool {
    let chars = pattern.chars().collect::<Vec<char>>();
    let words = str.split(" ").collect::<Vec<&str>>();

    let mut hashmap = HashMap::<&char, &str>::new();
    let mut reversed_hashmap = HashMap::<&str, &char>::new();
    let mut idx = 0;

    loop {
        match (chars.get(idx), words.get(idx)) {
            (Some(c), Some(w)) => {
                if let Some(prev) = hashmap.get(&c) {
                    if prev != w {
                        return false;
                    }
                } else if let Some(&prev) = reversed_hashmap.get(w) {
                    if prev != c {
                        return false;
                    }
                } else {
                    hashmap.insert(c, w);
                    reversed_hashmap.insert(w, c);
                }
            }
            (Some(_), None) => return false,
            (None, Some(_)) => return false,
            (None, None) => return true,
        }
        idx += 1;
    }
}

fn main() {
    assert_eq!(
        word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
        true
    );
    assert_eq!(
        word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
        false
    );
    assert_eq!(
        word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
        false
    );
}
