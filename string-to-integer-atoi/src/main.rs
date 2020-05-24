fn update_result(num: i32, is_negative: bool, result: &mut i32) {
    *result = i32::checked_mul(*result, 10)
        .and_then(|a| {
            if is_negative {
                return i32::checked_sub(a, num);
            } else {
                return i32::checked_add(a, num);
            }
        })
        .unwrap_or({
            if is_negative {
                i32::min_value()
            } else {
                i32::max_value()
            }
        });
}

pub fn my_atoi(str: String) -> i32 {
    let chars: Vec<char> = str.chars().collect();
    let len = chars.len();

    let mut is_negative = false;
    let mut into_body = false;
    let mut result = 0;
    for cur in 0..len {
        let val = &chars[cur];
        if !into_body {
            match &val {
                ' ' => continue,
                '-' => {
                    into_body = true;
                    is_negative = true;
                }
                '+' => {
                    into_body = true;
                }
                '0'..='9' => {
                    into_body = true;
                    update_result(val.to_digit(10).unwrap() as i32, is_negative, &mut result);
                }
                _ => return result,
            };
        } else {
            match &val {
                '0'..='9' => {
                    update_result(val.to_digit(10).unwrap() as i32, is_negative, &mut result);
                }
                _ => break,
            }
        }
    }

    result
}

fn main() {
    assert_eq!(my_atoi(String::from("  0000000000012345678")), 12345678);
    assert_eq!(my_atoi(String::from("42")), 42);
    assert_eq!(my_atoi(String::from("   -42")), -42);
    assert_eq!(my_atoi(String::from("4193 with words")), 4193);
}
