pub struct Solution;
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1 == "" {
            return str2;
        }
        if str2 == "" {
            return str1;
        }
        if str1.len() < str2.len() {
            return Self::gcd_of_strings(str2, str1);
        }
        if str1[..str2.len()] == str2 {
            return Self::gcd_of_strings(str1[str2.len()..].to_owned(), str2);
        }
        return "".to_owned();
    }
}
