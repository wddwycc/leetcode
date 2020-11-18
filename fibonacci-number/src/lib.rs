pub struct Solution {}
impl Solution {
    pub fn fib(n: i32) -> i32 {
        fn calc_with_cache(n: i32, cache: &mut Vec<Option<i32>>) -> i32 {
            match n {
                0 => 0,
                1 => 1,
                _ => {
                    let v1 = match cache[n as usize - 1] {
                        Some(a) => a,
                        None => {
                            let rv = calc_with_cache(n - 1, cache);
                            cache[n as usize - 1] = Some(rv);
                            rv
                        }
                    };
                    let v2 = match cache[n as usize - 2] {
                        Some(a) => a,
                        None => {
                            let rv = calc_with_cache(n - 2, cache);
                            cache[n as usize - 2] = Some(rv);
                            rv
                        }
                    };
                    v1 + v2
                },
            }    
        }
        let mut cache: Vec<Option<i32>> = vec![None; n as usize];
        calc_with_cache(n, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }
}
