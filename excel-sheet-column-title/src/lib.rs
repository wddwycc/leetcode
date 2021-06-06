pub struct Solution;
impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        if column_number == 0 {
            return "".to_owned();
        }
        column_number -= 1;
        let mut ans = Self::convert_to_title(column_number / 26);
        let b = b'A' + (column_number % 26) as u8;
        ans.push(b as char);
        ans
    }
}
