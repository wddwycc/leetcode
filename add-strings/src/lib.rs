pub struct Solution;
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1 = num1.into_bytes();
        let num2 = num2.into_bytes();
        let m = num1.len();
        let n = num2.len();

        let mut i = 0;
        let mut j = 0;
        let mut digits = vec![];
        let mut carry = 0;
        while m - 1 >= i || n - 1 >= j || carry > 0 {
            let mut sum = 0;
            if m - 1 >= i {
                sum += num1[m - 1 - i] - b'0';
                i += 1;
            }
            if n - 1 >= j {
                sum += num2[n - 1 - j] - b'0';
                j += 1;
            }
            sum += carry;
            carry = sum / 10;
            digits.push(sum % 10 + b'0');
        }
        digits.reverse();
        std::str::from_utf8(&digits).unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
