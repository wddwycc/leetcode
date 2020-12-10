pub struct Solution;
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut cur = 0;
        let mut on_left = true;
        loop {
            if cur == 1 {
                if arr[cur] <= arr[cur - 1] {
                    return false;
                }
            }
            if cur > 1 {
                if on_left {
                    if arr[cur] < arr[cur - 1] {
                        on_left = false;
                    } else if arr[cur] == arr[cur - 1] {
                        return false;
                    }
                } else {
                    if arr[cur] >= arr[cur - 1] {
                        return false;
                    }
                }
            }
            if cur == arr.len() - 1 {
                break;
            }
            cur += 1;
        }
        on_left == false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
