pub struct Solution;
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0; n]; n];

        let mut row_start = 0;
        let mut row_end = n - 1;
        let mut column_start = 0;
        let mut column_end = n - 1;

        let mut num = 1;
        loop {
            // NOTE: (row_start, column_start..=column_end)
            for c in column_start..=column_end {
                res[row_start][c] = num;
                num += 1;
            }
            row_start += 1;
            if row_start > row_end {
                break;
            }
            // NOTE: (row_start..=row_end, column_end)
            for r in row_start..=row_end {
                res[r][column_end] = num;
                num += 1;
            }
            column_end -= 1;
            // NOTE: (row_end, column_end..=column_start)
            for c in (column_start..=column_end).rev() {
                res[row_end][c] = num;
                num += 1;
            }
            row_end -= 1;
            if row_start > row_end {
                break;
            }
            // NOTE: (row_end..=row_start, column_start)
            for r in (row_start..=row_end).rev() {
                res[r][column_start] = num;
                num += 1;
            }
            column_start += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
