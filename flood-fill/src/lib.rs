use std::collections::{HashSet, VecDeque};

pub struct Solution;
impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let r = image.len();
        let c = image[0].len();
        let v = image[sr as usize][sc as usize];

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((sr as usize, sc as usize));

        while let Some(pixel) = queue.pop_front() {
            if image[pixel.0][pixel.1] != v {
                continue;
            }
            image[pixel.0][pixel.1] = new_color;
            let mut enqueue_if_needed = |pos: (usize, usize)| {
                if !visited.contains(&pos) && image[pos.0][pos.1] == v {
                    visited.insert(pos);
                    queue.push_back(pos);
                }
            };
            if pixel.0 > 0 {
                enqueue_if_needed((pixel.0 - 1, pixel.1));
            }
            if pixel.0 < r - 1 {
                enqueue_if_needed((pixel.0 + 1, pixel.1));
            }
            if pixel.1 > 0 {
                enqueue_if_needed((pixel.0, pixel.1 - 1));
            }
            if pixel.1 < c - 1 {
                enqueue_if_needed((pixel.0, pixel.1 + 1));
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
