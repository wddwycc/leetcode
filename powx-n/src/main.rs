use std::f64;

pub fn my_pow(x: f64, n: i32) -> f64 {
    if x == 1.0 {
        return x;
    }
    if x == -1.0 {
        if n % 2 == 0 {
            return 1.0;
        } else {
            return -1.0;
        }
    }
    let mut n = n;
    let mut res = x;
    while n != 1 && res != 0.0 {
        if n < 1 {
            n += 1;
            res = res / x;
        } else {
            n -= 1;
            res = res * x
        }
    }
    res
}

fn main() {
    println!("Hello, world!");
}
