pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for n in 1..=(num_rows as usize) {
        if n == 1 {
            result.push(vec![1]);
            continue;
        }
        if n == 2 {
            result.push(vec![1, 1]);
            continue;
        }
        let last_row = &result[n - 2];
        let mut row = vec![];
        for i in 0..n {
            if i == 0 || i == n - 1 {
                row.push(1);
            } else {
                row.push(last_row[i - 1] + last_row[i]);
            }
        }
        result.push(row);
    }
    result
}

fn main() {
    println!("Hello, world!");
}
