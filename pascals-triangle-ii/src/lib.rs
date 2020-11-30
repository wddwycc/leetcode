pub struct Solution {}
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        fn get_val(
            row: i32,
            column: i32,
            cache: &mut std::collections::HashMap<(i32, i32), i32>,
        ) -> i32 {
            if column == 0 || row == column {
                return 1;
            }
            if let Some(res) = cache.get(&(row, column)) {
                return *res;
            }
            let val = get_val(row - 1, column - 1, cache) + get_val(row - 1, column, cache);
            cache.insert((row, column), val);
            return val;
        }

        let mut cache = std::collections::HashMap::new();
        let mut res = vec![];
        for column in 0..=row_index {
            res.push(get_val(row_index, column, &mut cache))
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
