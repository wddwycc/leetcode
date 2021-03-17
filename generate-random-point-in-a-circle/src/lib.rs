use rand::{distributions::Uniform, prelude::ThreadRng, Rng};

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    rng: ThreadRng,
}

impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
            rng: rand::thread_rng(),
        }
    }

    fn rand_point(&mut self) -> Vec<f64> {
        let uniform = Uniform::from(-self.radius..=self.radius);
        loop {
            let x_offset = self.rng.sample(uniform);
            let y_offset = self.rng.sample(uniform);
            if (x_offset.powi(2) + y_offset.powi(2)).sqrt() > self.radius {
                continue;
            }
            return vec![self.x_center + x_offset, self.y_center + y_offset];
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
