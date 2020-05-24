pub fn check_unique(board: &Vec<char>) -> bool {
    let mut appeared = std::collections::HashSet::new();
    for &a in board {
        if a == '.' {
            continue;
        }
        if appeared.contains(&a) {
            return false;
        } else {
            appeared.insert(a);
        }
    }
    return true;
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // check rows
    for a in &board {
        if !check_unique(a) {
            return false;
        }
    }
    // check columns
    for column_idx in 0..9 {
        let column: Vec<char> = board.iter().map(|a| a[column_idx]).collect();
        if !check_unique(&column) {
            return false;
        }
    }
    // check blocks
    for i in 0..3 {
        for j in 0..3 {
            let mut vec = Vec::new();
            for a in (i * 3)..(i * 3 + 3) {
                for b in (j * 3)..(j * 3 + 3) {
                    vec.push(board[a][b]);
                }
            }
            if !check_unique(&vec) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let result = is_valid_sudoku(vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ]);
    assert_eq!(result, true);
}
