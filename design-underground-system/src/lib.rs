use std::collections::HashMap;

#[derive(Default)]
struct UndergroundSystem {
    // HashMap<id, (start_station, t)>
    pending_customers: HashMap<i32, (String, i32)>,
    // HashMap<(start_station, end_station), (total_time, total_counts)>
    avg_times: HashMap<(String, String), (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Default::default()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.pending_customers.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((start_station, prev_t)) = self.pending_customers.remove(&id) {
            let entry = self
                .avg_times
                .entry((start_station, station_name))
                .or_insert((0, 0));
            entry.0 += t - prev_t;
            entry.1 += 1;
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some((total_time, count)) = self.avg_times.get(&(start_station, end_station)) {
            return (*total_time as f64) / (*count as f64);
        }
        panic!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
