pub struct Range(i32, i32);
impl Range {
    pub fn repr(&self) -> String {
        if self.0 == self.1 {
            self.0.to_string()
        } else {
            [self.0.to_string(), "->".to_string(), self.1.to_string()].join("")
        }
    }
}

pub struct Solution;
impl Solution {
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        if nums.len() == 0 {
            return vec![Range(lower, upper).repr()];
        }

        let mut entities: Vec<Range> = vec![];
        for (idx, num) in nums.iter().enumerate() {
            if idx == 0 {
                if num != &lower {
                    entities.push(Range(lower, *num - 1));
                }
            }
            if idx > 0 {
                let prev_num = &nums[idx - 1];
                if prev_num != &(num - 1) {
                    entities.push(Range(prev_num + 1, num - 1));
                }
            }
            if idx == nums.len() - 1 {
                if num != &upper {
                    entities.push(Range(num + 1, upper));
                }
            }
        }
        entities.into_iter().map(|a| a.repr()).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
