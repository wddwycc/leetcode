enum State {
    NoAsk,
    Zero,
    One,
    ZeroAfterOne,
    OneAfterZero,
}

pub struct Solution;
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let bytes = s.into_bytes();

        let mut res = 0;
        for i in 0..bytes.len() {
            let mut state = State::NoAsk;
            let mut zeros = 0;
            let mut ones = 0;
            for j in i..bytes.len() {
                if bytes[j] == b'0' {
                    match state {
                        State::NoAsk => state = State::Zero,
                        State::One => state = State::ZeroAfterOne,
                        State::Zero | State::ZeroAfterOne => (),
                        State::OneAfterZero => break,
                    }
                    zeros += 1;
                }
                if bytes[j] == b'1' {
                    match state {
                        State::NoAsk => state = State::One,
                        State::Zero => state = State::OneAfterZero,
                        State::One | State::OneAfterZero => (),
                        State::ZeroAfterOne => break,
                    }
                    ones += 1;
                }
                match state {
                    State::ZeroAfterOne | State::OneAfterZero => {
                        if zeros == ones {
                            res += 1;
                        }
                    }
                    _ => (),
                }
            }
        }
        res
    }
}
