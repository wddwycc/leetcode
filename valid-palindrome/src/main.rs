pub fn is_palindrome(s: String) -> bool {
    let iter = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());
    iter.clone().eq(iter.rev())
}

fn main() {
    assert_eq!(
        is_palindrome(String::from("A man, a plan, a canal: Panama")),
        true
    );
}
