struct Solution;
impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut result = 0;
        for c in s.chars() {
            result = result * 26 + ((c as u8 - 64) % 27) as i32;
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
