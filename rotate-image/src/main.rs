// brute-force
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    let mut depth = 0;
    // rotate for each depth
    while depth < n / 2 {
        let start = depth;
        let end = n - 1 - depth;
        if start >= end {
            break;
        }
        // do the swap
        for i in start..end {
            let pt1 = matrix[start][i];
            let pt2 = matrix[i][end];
            let pt3 = matrix[end][end - (i - start)];
            let pt4 = matrix[end - (i - start)][start];
            println!("{}, {}, {}", start, end, i);
            println!("{}, {}, {}, {}", pt1, pt2, pt3, pt4);
            matrix[start][i] = pt4;
            matrix[i][end] = pt1;
            matrix[end][end - (i - start)] = pt2;
            matrix[end - (i - start)][start] = pt3;
        }
        depth += 1;
    }
}

fn main() {
    let mut ary = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    rotate(&mut ary);
    let result = vec![
        vec![15, 13, 2, 5],
        vec![14, 3, 4, 1],
        vec![12, 6, 8, 9],
        vec![16, 7, 10, 11],
    ];
    assert_eq!(ary, result);
}
