pub struct Solution;
impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let row_num = rooms.len();
        if row_num == 0 {
            return;
        }
        let column_num = rooms[0].len();
        if column_num == 0 {
            return;
        }

        let mut queue = {
            let mut res = std::collections::VecDeque::new();
            for i in 0..row_num {
                for j in 0..column_num {
                    if rooms[i][j] == 0 {
                        res.push_back((i, j));
                    }
                }
            }
            res
        };

        const ROOM_CODE: i32 = 2147483647;
        let mut step = 0;
        while !queue.is_empty() {
            step += 1;
            let queue_size = queue.len();
            for _ in 0..queue_size {
                let pos = queue.pop_front().unwrap();
                for pos_around in Self::positions_around(pos, row_num, column_num) {
                    if rooms[pos_around.0][pos_around.1] == ROOM_CODE {
                        rooms[pos_around.0][pos_around.1] = step;
                        queue.push_back(pos_around);
                    }
                }
            }
        }
    }

    fn positions_around(
        pos: (usize, usize),
        row_num: usize,
        column_num: usize,
    ) -> Vec<(usize, usize)> {
        let mut res = vec![];
        if pos.0 > 0 {
            res.push((pos.0 - 1, pos.1));
        }
        if pos.0 < row_num - 1 {
            res.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            res.push((pos.0, pos.1 - 1));
        }
        if pos.1 < column_num - 1 {
            res.push((pos.0, pos.1 + 1));
        }
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
