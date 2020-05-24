pub fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
}

fn main() {
    println!("Hello, world!");
}
