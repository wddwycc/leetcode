pub struct Solution;
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let n = dominoes.len();
        let dominoes: Vec<char> = dominoes.chars().collect();

        let mut ans = "".to_owned();
        let mut i = 0;
        let mut prev_is_r = false;
        while i < n {
            let mut j = i;
            while dominoes[j] == '.' && j + 1 < n {
                j += 1;
                continue;
            }

            if dominoes[j] == '.' {
                for _ in i..n {
                    ans.push(if prev_is_r { 'R' } else { '.' });
                }
            }

            if dominoes[j] == 'L' {
                if prev_is_r {
                    let mid = (i - 1 + j) / 2;
                    if (j - i) % 2 == 1 {
                        for k in i..=j {
                            if k < mid {
                                ans.push('R');
                            } else if k > mid {
                                ans.push('L');
                            } else {
                                ans.push('.')
                            }
                        }
                    } else {
                        for k in i..=j {
                            if k <= mid {
                                ans.push('R');
                            } else {
                                ans.push('L');
                            }
                        }
                    }
                } else {
                    for _ in i..=j {
                        ans.push('L');
                    }
                }
                prev_is_r = false;
            }

            if dominoes[j] == 'R' {
                if prev_is_r {
                    for _ in i..=j {
                        ans.push('R');
                    }
                } else {
                    for _ in i..j {
                        ans.push('.')
                    }
                    ans.push('R')
                }
                prev_is_r = true;
            }
            i = j + 1;
        }
        ans
    }
}
