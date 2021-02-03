pub struct Solution;
impl Solution {
    pub fn min_distance(
        height: i32,
        width: i32,
        tree: Vec<i32>,
        squirrel: Vec<i32>,
        nuts: Vec<Vec<i32>>,
    ) -> i32 {
        let base_disntance: i32 = nuts
            .iter()
            .map(|nut| (nut[0] - tree[0]).abs() + (nut[1] - tree[1]).abs())
            .sum();
        // find the one to go first and the one to end
        let min_offset: i32 = nuts
            .iter()
            .map(|nut| {
                let tree_distance = (nut[0] - tree[0]).abs() + (nut[1] - tree[1]).abs();
                let squirrel_distance = (nut[0] - squirrel[0]).abs() + (nut[1] - squirrel[1]).abs();
                squirrel_distance - tree_distance
            })
            .min()
            .unwrap();
        base_disntance * 2 + min_offset
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
