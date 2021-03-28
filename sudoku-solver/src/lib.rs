pub struct Solution;
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::backtracking(board, (0, 0));
    }

    fn backtracking(board: &mut Vec<Vec<char>>, pos: (usize, usize)) -> bool {
        let (x, y) = pos;
        if board[x][y] != '.' {
            if let Some(next_pos) = Self::next_pos(pos) {
                return Self::backtracking(board, next_pos);
            } else {
                return true;
            }
        }

        let mut options = vec![true; 9];
        for i in 0..9 {
            if let Some(digit) = board[i][y].to_digit(10) {
                options[digit as usize - 1] = false;
            }
            if let Some(digit) = board[x][i].to_digit(10) {
                options[digit as usize - 1] = false;
            }
        }
        for i in (x / 3 * 3)..=(x / 3 * 3 + 2) {
            for j in (y / 3 * 3)..=(y / 3 * 3 + 2) {
                if let Some(digit) = board[i][j].to_digit(10) {
                    options[digit as usize - 1] = false;
                }
            }
        }
        for i in 0..9 {
            if !options[i] {
                continue;
            }
            board[x][y] = std::char::from_digit((i + 1) as u32, 10).unwrap();
            if let Some(next_pos) = Self::next_pos(pos) {
                let res = Self::backtracking(board, next_pos);
                if res {
                    return true;
                }
            } else {
                return true;
            }
        }
        board[x][y] = '.';
        false
    }

    fn next_pos(pos: (usize, usize)) -> Option<(usize, usize)> {
        if pos.1 < 8 {
            Some((pos.0, pos.1 + 1))
        } else if pos.0 < 8 {
            Some((pos.0 + 1, 0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::next_pos((1, 2)), Some((1, 3)));
        assert_eq!(Solution::next_pos((0, 8)), Some((1, 0)));
        assert_eq!(Solution::next_pos((8, 8)), None);
    }
}
