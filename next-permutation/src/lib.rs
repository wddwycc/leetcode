pub struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut cur = nums.len() - 1;
        let mut heap = std::collections::BinaryHeap::new();
        loop {
            if let Some(&(max, _)) = heap.peek() {
                // find the min previous element greater than nums[cur]
                // if exist, swap then sort.
                // else, next iteration
                if max > nums[cur] {
                    let mut m = heap.pop().unwrap();
                    while let Some(v) = heap.pop() {
                        if v.0 > nums[cur] {
                            m = v;
                        } else {
                            break;
                        }
                    }
                    nums.swap(cur, m.1);
                    nums[(cur + 1)..].sort();
                    return;
                } else {
                    heap.push((nums[cur], cur));
                }
            } else {
                heap.push((nums[cur], cur));
            }

            if cur == 0 {
                nums.sort();
                break;
            } else {
                cur -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
