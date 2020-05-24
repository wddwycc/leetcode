pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    if len < 2 {
        return;
    }

    let mut i = 0;
    let mut j = len - 1;

    while i < j {
        let tmp = s[i];
        s[i] = s[j];
        s[j] = tmp;

        i += 1;
        j -= 1;
    }
}

fn main() {
    println!("Hello, world!");
}
