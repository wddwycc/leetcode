use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendar {
    tree: BTreeMap<i32, i32>,
}

impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for (&s, _) in self.tree.range(start..).take(1) {
            if s < end {
                return false;
            }
        }
        for (_, &e) in self.tree.range(..start).rev().take(1) {
            if e > start {
                return false;
            }
        }
        self.tree.insert(start, end);
        true
    }
}
