pub struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len() - 1;
        let n = matrix[0].len() - 1;

        let mut x = m;
        let mut y = 0;
        loop {
            if matrix[x][y] == target {
                return true;
            } else if matrix[x][y] > target && x > 0 {
                x -= 1;
            } else if matrix[x][y] < target && y < n {
                y += 1;
            } else {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
