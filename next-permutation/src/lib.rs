pub struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut heap = std::collections::BinaryHeap::new();
        for i in (0..nums.len()).rev() {
            let max = match heap.peek() {
                Some(&(a, _)) => a,
                None => {
                    heap.push((nums[i], i));
                    continue;
                }
            };
            // find the min previous element greater than nums[cur]
            // if exist, swap, sort, return
            if max > nums[i] {
                let (_, to_swap) = heap.into_iter().filter(|a| a.0 > nums[i]).min().unwrap();
                nums.swap(i, to_swap);
                nums[(i + 1)..].sort();
                return;
            } else {
                heap.push((nums[i], i));
            }
        }
        nums.sort();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
