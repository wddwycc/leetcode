use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut res = HashSet::new();
        if x == 1 && y == 1 {
            let mut res = vec![];
            if bound >= 2 {
                res.push(2);
            }
            return res;
        }
        if x == 1 {
            return Self::powerful_integers_single(y, bound);
        }
        if y == 1 {
            return Self::powerful_integers_single(x, bound);
        }

        'outer: for i in 0.. {
            for j in 0.. {
                let v = x.pow(i) + y.pow(j);
                if v <= bound {
                    res.insert(v);
                } else {
                    if j == 0 {
                        break 'outer;
                    } else {
                        break;
                    }
                }
            }
        }
        res.into_iter().collect()
    }

    fn powerful_integers_single(x: i32, bound: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 0.. {
            let v = x.pow(i) + 1;
            if v <= bound {
                res.push(v);
            } else {
                break;
            }
        }
        res
    }
}
