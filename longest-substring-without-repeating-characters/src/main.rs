use std::cmp;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut len = 0;
    let mut i = 0;
    let mut map = HashMap::new();
    for (j, c) in s.char_indices() {
        if let Some(last) = map.insert(c, j + 1) {
            i = cmp::max(i, last)
        }
        len = cmp::max(len, j - i + 1);
    }
    len as i32
}

fn main() {
    assert_eq!(length_of_longest_substring(String::from("  ")), 1);
}
