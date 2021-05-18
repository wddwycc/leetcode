use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut res: HashMap<String, Vec<String>> = HashMap::new();
        for path in paths {
            let parts = path.split(" ").collect::<Vec<_>>();
            let p1 = parts[0].clone();
            for i in 1..parts.len() {
                let parts2 = parts[i].split("(").collect::<Vec<_>>();
                let p2 = parts2[0];
                let content = parts2[1].split(")").collect::<Vec<_>>()[0].to_owned();
                res.entry(content)
                    .or_insert(vec![])
                    .push(format!("{}/{}", p1, p2));
            }
        }
        res.into_iter()
            .filter(|(_, v)| v.len() > 1)
            .map(|(_, v)| v)
            .collect()
    }
}
