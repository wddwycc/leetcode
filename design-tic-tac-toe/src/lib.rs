struct TicTacToe {
    n: usize,
    board: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TicTacToe {
    /** Initialize your data structure here. */
    fn new(n: i32) -> Self {
        let n = n as usize;
        Self {
            n,
            board: vec![vec![0; n]; n],
        }
    }

    /** Player {player} makes a move at ({row}, {col}).
    @param row The row of the board.
    @param col The column of the board.
    @param player The player, can be either 1 or 2.
    @return The current winning condition, can be either:
            0: No one wins.
            1: Player 1 wins.
            2: Player 2 wins. */
    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        self.board[row as usize][col as usize] = player;
        // check each row, each column and each diagonal
        for &p in [1, 2].iter() {
            for r in 0..self.n {
                if self.board[r].iter().all(|&a| a == p) {
                    return p;
                }
            }
            'outer: for c in 0..self.n {
                for r in 0..self.n {
                    if self.board[r][c] == p {
                        continue;
                    } else {
                        continue 'outer;
                    }
                }
                return p;
            }
            if (0..self.n).into_iter().all(|i| self.board[i][i] == p) {
                return p;
            }
            if (0..self.n)
                .into_iter()
                .all(|i| self.board[i][self.n - 1 - i] == p)
            {
                return p;
            }
        }
        0
    }
}
