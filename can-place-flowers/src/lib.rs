pub struct Solution;
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut to_place = n;
        let mut bed = flowerbed;
        let bed_len = bed.len();

        if bed_len == 1 && n == 1 {
            return bed[0] == 0;
        }

        if bed_len >= 2 {
            if bed[0] == 0 && bed[1] == 0 {
                bed[0] = 1;
                to_place -= 1;
            }
            if bed[bed_len - 1] == 0 && bed[bed_len - 2] == 0 {
                bed[bed_len - 1] = 1;
                to_place -= 1;
            }
        }
        // NOTE: Find all continuous 3x0
        let mut cur: usize = 0;
        loop {
            if to_place <= 0 {
                return true;
            }
            if cur + 3 >= bed_len {
                return false;
            }
            if bed[cur] == 0 && bed[cur + 1] == 0 && bed[cur + 2] == 0 {
                bed[cur + 1] = 1;
                to_place -= 1;
                cur += 2;
            } else {
                cur += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
        assert_eq!(Solution::can_place_flowers(vec![1], 1), false);
    }
}
