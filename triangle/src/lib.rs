pub struct Solution;
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len() - 1;

        // given function f(i, j) -> i32
        // represents at pos (i, j), min path sum to reach the bottom
        // when i < n
        //   f(i, j) = triangle[i][j] + min { f(i + 1, j), f(i + 1, j + 1) }
        // else
        //   f(i, j) = triangle[i][j]

        // let's do in-place bottom-up dp
        for i in (0..=n).rev() {
            if i == n {
                continue;
            }
            for j in 0..=i {
                triangle[i][j] += std::cmp::min(triangle[i + 1][j], triangle[i + 1][j + 1]);
            }
        }
        triangle[0][0]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
