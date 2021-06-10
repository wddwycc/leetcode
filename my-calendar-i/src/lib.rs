#[derive(Default)]
struct MyCalendar {
    ranges: Vec<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(prev_start, prev_end) in self.ranges.iter() {
            if prev_start >= end || prev_end <= start {
                continue;
            }
            return false;
        }
        self.ranges.push((start, end));
        true
    }
}
