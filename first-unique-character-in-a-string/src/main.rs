use std::collections::HashMap;

pub fn first_uniq_char(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut dist: HashMap<char, usize> = HashMap::new();
    for &c in &chars {
        *dist.entry(c).or_insert(0) += 1;
    }
    for i in 0..len {
        match dist.get(&chars[i]) {
            Some(&a) if a == 1 => {
                return i as i32;
            }
            _ => (),
        }
    }
    -1
}

fn main() {
    assert_eq!(first_uniq_char(String::from("leetcode")), 0);
    println!("Hello, world!");
}
