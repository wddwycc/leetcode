#[derive(Default)]
struct MyCalendar {
    ranges: Vec<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &range in self.ranges.iter() {
            if range.0 >= end || range.1 <= start {
                continue;
            }
            return false;
        }
        self.ranges.push((start, end));
        true
    }
}
