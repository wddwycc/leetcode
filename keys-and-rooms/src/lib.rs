use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn can_visit_all_rooms(mut rooms: Vec<Vec<i32>>) -> bool {
        // DFS, O(n)
        let mut visited = HashSet::new();
        visited.insert(0);
        let mut stack = vec![];
        stack.append(&mut rooms[0]);
        while let Some(idx) = stack.pop() {
            if visited.contains(&idx) {
                break;
            }
            visited.insert(idx);
            stack.append(&mut rooms[idx as usize]);
        }
        // validate result, O(n)
        for n in 0..rooms.len() {
            if !visited.contains(&(n as i32)) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
