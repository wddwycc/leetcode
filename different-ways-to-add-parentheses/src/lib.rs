pub struct Solution;
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut nums = vec![];
        let mut operators = vec![];
        let mut tmp = 0;
        for c in expression.chars() {
            if c == '+' || c == '-' || c == '*' {
                operators.push(c);
                nums.push(tmp);
                tmp = 0;
            } else {
                tmp *= 10;
                tmp += c as i32 - '0' as i32;
            }
        }
        nums.push(tmp);

        Self::calc(&nums, &operators, 0, nums.len() - 1)
    }

    fn calc(nums: &[i32], operators: &[char], l: usize, r: usize) -> Vec<i32> {
        if l == r {
            return vec![nums[l]];
        }
        let mut res = vec![];
        for i in l..r {
            let l_values = Self::calc(nums, operators, l, i);
            let r_values = Self::calc(nums, operators, i + 1, r);
            for j in 0..l_values.len() {
                for k in 0..r_values.len() {
                    let v = match operators[i] {
                        '+' => l_values[j] + r_values[k],
                        '-' => l_values[j] - r_values[k],
                        '*' => l_values[j] * r_values[k],
                        _ => panic!(),
                    };
                    res.push(v);
                }
            }
        }
        res
    }
}
