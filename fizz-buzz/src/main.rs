pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result = vec![];

    for i in 1..=n {
        let m3 = i % 3 == 0;
        let m5 = i % 5 == 0;
        let value = match (m3, m5) {
            (true, true) => String::from("FizzBuzz"),
            (true, false) => String::from("Fizz"),
            (false, true) => String::from("Buzz"),
            _ => i.to_string(),
        };
        result.push(value);
    }

    result
}

fn main() {
    println!("Hello, world!");
}
