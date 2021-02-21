pub struct Solution;
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::merge_sort(&nums)
    }

    fn merge_sort(src: &[i32]) -> Vec<i32> {
        if src.len() <= 1 {
            return src.to_vec();
        }

        let mid = src.len() / 2;
        let pt1 = Self::merge_sort(&src[..mid]);
        let pt2 = Self::merge_sort(&src[mid..src.len()]);
        // NOTE: merge pt1, pt2
        let mut res = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < pt1.len() || j < pt2.len() {
            if i < pt1.len() && j < pt2.len() {
                if pt1[i] < pt2[j] {
                    res.push(pt1[i]);
                    i += 1;
                } else {
                    res.push(pt2[j]);
                    j += 1;
                }
            } else if i < pt1.len() {
                res.push(pt1[i]);
                i += 1;
            } else if j < pt2.len() {
                res.push(pt2[j]);
                j += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    }
}
