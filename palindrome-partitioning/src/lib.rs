pub enum Res {
    Success(Vec<Vec<String>>),
    End,
    Failure,
}

pub struct Solution;
impl Solution {
    fn helper(s: String) -> Res {
        if s.len() == 0 {
            return Res::End;
        }
        if s.len() == 1 {
            return Res::Success(vec![vec![s]]);
        }

        let mut results = vec![];
        for i in 0..s.len() {
            let original = String::from(&s[0..=i]);
            let rev: String = (&s[0..=i]).clone().chars().rev().collect();
            if original == rev {
                let rest = String::from(&s[(i + 1)..]);
                match Self::helper(rest) {
                    Res::Success(lst) => {
                        for mut a in lst {
                            let mut res = vec![];
                            res.push(original.clone());
                            res.append(&mut a);
                            results.push(res);
                        }
                    }
                    Res::Failure => (),
                    Res::End => results.push(vec![original]),
                }
            }
        }
        if results.len() == 0 {
            return Res::Failure;
        } else {
            return Res::Success(results);
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        match Self::helper(s) {
            Res::Success(a) => a,
            Res::Failure => vec![],
            Res::End => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
