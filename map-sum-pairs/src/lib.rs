use std::collections::HashMap;

#[derive(Default)]
pub struct MapSum {
    pub children: [Option<Box<MapSum>>; 26],
    pub values: HashMap<String, i32>
}

impl MapSum {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn insert(&mut self, key: String, val: i32) {
        let mut cur = self;
        for c in key.chars() {
            let idx = (c as i8 - 'a' as i8) as usize;
            cur = cur.children[idx].get_or_insert_with(|| Box::new(MapSum::new()));
            cur.values.insert(key.clone(), val);
        }
    }
    
    pub fn sum(&self, prefix: String) -> i32 {
        let mut cur = self;
        for c in prefix.chars() {
            let idx = (c as i8 - 'a' as i8) as usize;
            match cur.children[idx].as_ref() {
                Some(a) => cur = a,
                None => return 0
            }
        }
        cur.values.iter().map(|s| s.1).sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut ms = MapSum::new();
        ms.insert("apple".to_string(), 3);
        assert_eq!(ms.sum("ap".to_string()), 3);
        ms.insert("ap".to_string(), 2);
        assert_eq!(ms.sum("ap".to_string()), 5);
    }
}
