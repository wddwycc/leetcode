use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // given function to represent largest rectangle area from idx i to j
        // f(i, j) = max { f(i + 1, j), f(i, j - 1), min { h[i] h[i + 1], ...h[j] } * (j - i)  }
        // f(i, i) = h(i)

        // let's try top-down dp
        let mut res_cache: HashMap<(usize, usize), i32> = HashMap::new();
        let mut min_cache: HashMap<(usize, usize), i32> = HashMap::new();
        Self::max_area(
            &heights,
            (0, heights.len() - 1),
            &mut res_cache,
            &mut min_cache,
        )
    }

    fn max_area(
        heights: &[i32],
        range: (usize, usize),
        res_cache: &mut HashMap<(usize, usize), i32>,
        min_cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        let (i, j) = range;
        if i == j {
            return heights[i];
        }

        if let Some(cached) = res_cache.get(&range) {
            return *cached;
        }
        let mut res = 0;
        res = res.max(Self::max_area(heights, (i + 1, j), res_cache, min_cache));
        res = res.max(Self::max_area(heights, (i, j - 1), res_cache, min_cache));
        res = res.max(Self::min_avg(heights, range, min_cache) * (j - i + 1) as i32);

        res_cache.insert((i, j), res);
        res
    }

    fn min_avg(
        heights: &[i32],
        range: (usize, usize),
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        let (i, j) = range;
        if i == j {
            return heights[i];
        }
        if let Some(cached) = cache.get(&range) {
            return *cached;
        }
        let res = std::cmp::min(Self::min_avg(heights, (i + 1, j), cache), heights[i]);
        cache.insert(range, res);
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
