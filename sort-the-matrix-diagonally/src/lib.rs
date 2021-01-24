pub struct Solution;
impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        // for start_positions in (0, 1) ..= (0, n - 1) & (1, 0) ..= (m - 1, 0) & (0, 0)
        //  for (x, y) in start_positions
        //    every step: (x, y) -> (x + 1, y + 1)
        //    till `x` reached `n - 1` or `y` reached `m - 1`
        let start_positions = {
            let mut res = vec![];
            for k in 1..n {
                res.push((0, k));
            }
            for k in 1..m {
                res.push((k, 0));
            }
            res.push((0, 0));
            res
        };
        for (mut x, mut y) in start_positions {
            let mut positions = vec![];
            while !(x == m || y == n) {
                positions.push((x, y));
                x += 1;
                y += 1;
            }
            let mut values: Vec<i32> = positions.iter().map(|(l, r)| mat[*l][*r]).collect();
            values.sort();
            for cur in 0..positions.len() {
                let (x, y) = positions[cur];
                mat[x][y] = values[cur];
            }
        }
        mat
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
