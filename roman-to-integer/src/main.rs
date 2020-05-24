fn process_digit(
    chars: &Vec<char>,
    len: usize,
    cur: &mut usize,
    rv: &mut usize,
    symbol1: (char, usize),
    symbol2: (char, usize),
    symbol3: (char, usize),
) -> bool {
    if chars[*cur] == symbol2.0 {
        *rv += symbol2.1;
        *cur += 1;
        return true;
    }
    if chars[*cur] == symbol3.0 {
        *rv += symbol3.1;
        *cur += 1;
        return true;
    }
    if chars[*cur] == symbol1.0 {
        if *cur + 1 < len {
            if chars[*cur + 1] == symbol2.0 {
                *rv += symbol2.1 - symbol1.1;
                *cur += 2;
                return true;
            }
            if chars[*cur + 1] == symbol3.0 {
                *rv += symbol3.1 - symbol1.1;
                *cur += 2;
                return true;
            }
        }
        *rv += symbol1.1;
        *cur += 1;
        return true;
    }
    return false;
}

pub fn roman_to_int(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    let mut rv = 0;
    let mut cur = 0;

    let mut phase = 0;
    while cur < len {
        if phase <= 0 {
            let processed = process_digit(
                &chars,
                len,
                &mut cur,
                &mut rv,
                ('C', 100),
                ('D', 500),
                ('M', 1_000),
            );
            if processed {
                continue;
            } else {
                phase += 1;
            }
        }
        if phase <= 1 {
            let processed = process_digit(
                &chars,
                len,
                &mut cur,
                &mut rv,
                ('X', 10),
                ('L', 50),
                ('C', 100),
            );
            if processed {
                continue;
            } else {
                phase += 1;
            }
        }
        if phase <= 2 {
            let processed = process_digit(
                &chars,
                len,
                &mut cur,
                &mut rv,
                ('I', 1),
                ('V', 5),
                ('X', 10),
            );
            if processed {
                continue;
            } else {
                phase += 1;
            }
        }
    }

    rv as i32
}

fn main() {
    assert_eq!(roman_to_int(String::from("VII")), 7);
}
