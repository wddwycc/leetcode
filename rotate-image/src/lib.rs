pub struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for offset in 0..(n / 2) {
            let w = n - offset * 2;
            for i in 0..(w - 1) {
                let v1 = matrix[offset][offset + i];
                let v2 = matrix[offset + i][offset + w - 1];
                let v3 = matrix[offset + w - 1][offset + w - 1 - i];
                let v4 = matrix[offset + w - 1 - i][offset];
                matrix[offset][offset + i] = v4;
                matrix[offset + i][offset + w - 1] = v1;
                matrix[offset + w - 1][offset + w - 1 - i] = v2;
                matrix[offset + w - 1 - i][offset] = v3;
            }
        }
    }
}
