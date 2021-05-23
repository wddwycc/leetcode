pub struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.into_bytes();
        let mut indice = vec![];
        // find all vowel indice
        for i in 0..s.len() {
            if s[i] == b'a'
                || s[i] == b'e'
                || s[i] == b'i'
                || s[i] == b'o'
                || s[i] == b'u'
                || s[i] == b'A'
                || s[i] == b'E'
                || s[i] == b'I'
                || s[i] == b'O'
                || s[i] == b'U'
            {
                indice.push(i);
            }
        }
        if indice.len() == 0 {
            return String::from_utf8(s).unwrap();
        }
        // two-pointers swap
        let mut i = 0;
        let mut j = indice.len() - 1;
        while i < j {
            s.swap(indice[i], indice[j]);
            i += 1;
            j -= 1;
        }
        String::from_utf8(s).unwrap()
    }
}
