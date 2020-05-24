/**
 * 1.     1
 * 2.     11
 * 3.     21
 * 4.     1211
 * 5.     111221
*/

pub struct Solution {}
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return String::from("1");
        }
        if n == 2 {
            return String::from("11");
        }
        let prev_res: Vec<char> = Self::count_and_say(n - 1).chars().collect();

        let mut prev_chr = &prev_res[0];
        let mut count = 1;
        let mut res = vec![];

        for chr in &prev_res[1..] {
            if chr == prev_chr {
                count += 1;
            } else {
                res.push(format!("{}{}", count, prev_chr));
                count = 1;
            }
            prev_chr = chr;
        }
        res.push(format!("{}{}", count, prev_chr));
        res.join("")
    }
}

fn main() {
    assert_eq!(Solution::count_and_say(4), String::from("1211"));
}
