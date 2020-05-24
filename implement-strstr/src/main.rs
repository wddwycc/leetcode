pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack == "" && needle == "" {
        return 0;
    }
    if haystack == "" {
        return -1;
    }
    if needle == "" {
        return 0;
    }

    let haystack: Vec<char> = haystack.chars().collect();
    let needle: Vec<char> = needle.chars().collect();

    let mut i = 0;
    let mut j = 0;
    while i < haystack.len() {
        if haystack[i] == needle[j] {
            if j == needle.len() - 1 {
                return (i + 1 - needle.len()) as i32;
            }
            j += 1;
            i += 1;
        } else {
            if j > 0 {
                i -= j - 1;
                j = 0;
            } else {
                i += 1;
            }
        }
    }
    -1
}

fn main() {
    assert_eq!(
        str_str(String::from("mississippi"), String::from("issip")),
        4
    );
}
