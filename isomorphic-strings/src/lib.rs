use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut hm1 = HashMap::new();
        let mut hm2 = HashMap::new();
        let s = s.into_bytes();
        let t = t.into_bytes();
        for i in 0..s.len() {
            let l = s[i];
            let r = t[i];
            if let Some(&prev) = hm1.get(&l) {
                if prev != r { return false; }
            } else {
                hm1.insert(l, r);
            }
            if let Some(&prev) = hm2.get(&r) {
                if prev != l { return false; }
            } else {
                hm2.insert(r, l);
            }
            
        }
        true
    }
}