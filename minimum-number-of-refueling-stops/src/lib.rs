use std::collections::BinaryHeap;

pub struct Solution;
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        // Greedy solution:
        // while not able to reach target, move forward as far as it can, collect stations passby.
        // if cannot move forward anymore, try consume previous station with max fuel
        let mut pos = 0;
        let mut fuel = start_fuel;
        let mut ans = 0;
        let mut available_fuels = BinaryHeap::new();
        let mut station_idx = 0;
        while pos + fuel < target {
            pos += fuel;
            fuel = 0;
            while station_idx < stations.len() && stations[station_idx][0] <= pos {
                available_fuels.push(stations[station_idx][1]);
                station_idx += 1;
            }
            if let Some(fuel_to_add) = available_fuels.pop() {
                fuel += fuel_to_add;
                ans += 1;
            } else {
                return -1;
            }
        }
        ans
    }
}
