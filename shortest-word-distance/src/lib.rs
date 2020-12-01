pub struct Solution {}
impl Solution {
    pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let word1_indices: Vec<i32> = words
            .iter()
            .enumerate()
            .filter(|(_, w)| w == &&word1)
            .map(|(idx, _)| idx as i32)
            .collect();
        let word2_indices: Vec<i32> = words
            .iter()
            .enumerate()
            .filter(|(_, w)| w == &&word2)
            .map(|(idx, _)| idx as i32)
            .collect();
        let mut min_distance = (word1_indices[0] - word2_indices[0]).abs();
        let mut cur1 = 0;
        let mut cur2 = 0;
        loop {
            let distance = word1_indices[cur1] - word2_indices[cur2];
            let distance_abs = distance.abs();
            if distance_abs < min_distance {
                min_distance = distance_abs;
            }
            if word1_indices[cur1] > word2_indices[cur2] {
                if cur2 < word2_indices.len() - 1 {
                    cur2 += 1;
                } else {
                    break;
                }
            } else {
                if cur1 < word1_indices.len() - 1 {
                    cur1 += 1;
                } else {
                    break;
                }
            }
        }

        min_distance
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
