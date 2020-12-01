pub struct Solution {}
impl Solution {
    fn digit(num: i32) -> Option<String> {
        match num {
            0 => None,
            1 => Some("One".into()),
            2 => Some("Two".into()),
            3 => Some("Three".into()),
            4 => Some("Four".into()),
            5 => Some("Five".into()),
            6 => Some("Six".into()),
            7 => Some("Seven".into()),
            8 => Some("Eight".into()),
            9 => Some("Nine".into()),
            _ => panic!(),
        }
    }

    fn two_digits(num1: i32, num2: i32) -> Option<String> {
        match (num1, num2) {
            (0, num2) => Self::digit(num2),
            (1, 0) => Some("Ten".into()),
            (1, 1) => Some("Eleven".into()),
            (1, 2) => Some("Twelve".into()),
            (1, 3) => Some("Thirteen".into()),
            (1, 4) => Some("Fourteen".into()),
            (1, 5) => Some("Fifteen".into()),
            (1, 6) => Some("Sixteen".into()),
            (1, 7) => Some("Seventeen".into()),
            (1, 8) => Some("Eighteen".into()),
            (1, 9) => Some("Nineteen".into()),
            (fst, snd) => {
                let fst_repr: String = match fst {
                    2 => "Twenty".into(),
                    3 => "Thirty".into(),
                    4 => "Forty".into(),
                    5 => "Fifty".into(),
                    6 => "Sixty".into(),
                    7 => "Seventy".into(),
                    8 => "Eighty".into(),
                    9 => "Ninety".into(),
                    _ => panic!(),
                };
                if let Some(snd_repr) = Self::digit(snd) {
                    return Some(format!("{} {}", fst_repr, snd_repr));
                } else {
                    return Some(fst_repr.into());
                };
            }
        }
    }

    fn three_digits(num1: i32, num2: i32, num3: i32) -> Option<String> {
        match Self::digit(num1) {
            None => Self::two_digits(num2, num3),
            Some(fst_repr) => {
                if let Some(snd_repr) = Self::two_digits(num2, num3) {
                    return Some(format!("{} Hundred {}", fst_repr, snd_repr));
                } else {
                    return Some(format!("{} Hundred", fst_repr));
                };
            }
        }
    }

    fn number_to_hundred(num: i32) -> Option<String> {
        match num {
            a @ 0..=9 => Self::digit(a),
            a @ 10..=99 => Self::two_digits(a / 10, a % 10),
            a @ 100..=999 => Self::three_digits(a / 100, (a - (a / 100 * 100)) / 10, a % 10),
            _ => panic!(),
        }
    }

    pub fn number_to_words(num: i32) -> String {
        let res = match num {
            0 => vec![Some("Zero".to_string())],
            a @ 1..=999 => vec![Self::number_to_hundred(a)],
            a @ 1_000..=999_999 => vec![
                Self::number_to_hundred(a / 1_000),
                Some("Thousand".to_string()),
                Self::number_to_hundred(a % 1_000),
            ],
            a @ 1_000_000..=999_999_999 => vec![
                Self::number_to_hundred(a / 1_000_000),
                Some("Million".to_string()),
                Self::number_to_hundred((a - (a / 1_000_000 * 1_000_000)) / 1_000),
                {
                    if (a - (a / 1_000_000 * 1_000_000)) / 1_000 > 0 {
                        Some("Thousand".to_string())
                    } else {
                        None
                    }
                },
                Self::number_to_hundred(a % 1_000),
            ],
            a => vec![
                Self::number_to_hundred(a / 1_000_000_000),
                Some("Billion".to_string()),
                Self::number_to_hundred((a - (a / 1_000_000_000 * 1_000_000_000)) / 1_000_000),
                {
                    if (a - (a / 1_000_000_000 * 1_000_000_000)) / 1_000_000 > 0 {
                        Some("Million".to_string())
                    } else {
                        None
                    }
                },
                Self::number_to_hundred((a - (a / 1_000_000 * 1_000_000)) / 1_000),
                {
                    if (a - (a / 1_000_000 * 1_000_000)) / 1_000 > 0 {
                        Some("Thousand".to_string())
                    } else {
                        None
                    }
                },
                Self::number_to_hundred(a % 1_000),
            ],
        };
        let res: Vec<String> = res.into_iter().filter_map(|a| a).collect();
        res.join(" ")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
