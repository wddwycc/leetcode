pub fn hamming_weight(n: u32) -> i32 {
    let mut n = n;
    let mut result = 0;
    while n > 0 {
        result += 1;
        n = n & (n - 1);
    }
    result as i32
}

fn main() {
    println!("Hello, world!");
}
