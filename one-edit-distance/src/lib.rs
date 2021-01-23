pub struct Solution;
impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        Self::check_one_deletion(&s, &t)
            || Self::check_one_deletion(&t, &s)
            || Self::check_one_replace(&s, &t)
    }

    /// check if str1 delete one char results into str2
    pub fn check_one_deletion(s: &[char], t: &[char]) -> bool {
        let len1 = s.len();
        let len2 = t.len();
        if len1 != len2 + 1 {
            return false;
        }
        if len1 == 1 {
            return true;
        }
        let mut i = 0;
        let mut j = 0;
        let mut deleted = false;
        loop {
            if j == len2 {
                if deleted {
                    return i == len1;
                } else {
                    return i == len1 - 1;
                }
            }
            if s[i] == t[j] {
                i += 1;
                j += 1;
            } else {
                if deleted {
                    return false;
                }
                i += 1;
                deleted = true;
            }
        }
    }

    /// check if str1 replace one char results into str2
    pub fn check_one_replace(s: &[char], t: &[char]) -> bool {
        let len1 = s.len();
        let len2 = t.len();
        if len1 != len2 || len1 == 0 {
            return false;
        }
        let mut diff_count = 0;
        for cur in 0..len1 {
            if s[cur] != t[cur] {
                diff_count += 1;
            }
            if diff_count > 1 {
                return false;
            }
        }
        return diff_count == 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
