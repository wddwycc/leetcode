pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut k = m + n - 1;
    let mut m = m - 1;
    let mut n = n - 1;
    loop {
        let val1 = {
            if m >= 0 {
                Some(nums1[m as usize])
            } else {
                None
            }
        };
        let val2 = {
            if n >= 0 {
                Some(nums2[n as usize])
            } else {
                None
            }
        };
        match (val1, val2) {
            (Some(a), Some(b)) => {
                if a > b {
                    nums1[k as usize] = a;
                    m -= 1;
                } else {
                    nums1[k as usize] = b;
                    n -= 1;
                }
            }
            (Some(a), None) => {
                nums1[k as usize] = a;
                m -= 1;
            }
            (None, Some(a)) => {
                nums1[k as usize] = a;
                n -= 1;
            }
            (None, None) => break,
        }
        k -= 1;
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}
