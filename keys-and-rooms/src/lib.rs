use std::collections::HashSet;

pub struct Solution;
impl Solution {
    // DFS => time: O(n), space: O(n)
    pub fn can_visit_all_rooms(mut rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = HashSet::new();
        visited.insert(0);
        let mut stack = vec![];
        stack.append(&mut rooms[0]);
        while let Some(idx) = stack.pop() {
            if visited.contains(&idx) {
                continue;
            }
            visited.insert(idx);
            stack.append(&mut rooms[idx as usize]);
        }
        visited.len() == rooms.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
