pub struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Sudoku {
        Sudoku { board }
    }

    fn get_empty_cell(&self) -> Option<(usize, usize)> {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn check_if_pos_is_legal(&self, pos: (usize, usize), n: u8) -> bool {
        let (x, y) = pos;

        for i in 0..9 {
            if self.board[x][i] == n {
                return false;
            }
        }

        for i in 0..9 {
            if self.board[i][y] == n {
                return false;
            }
        }

        let board_start_x = (x / 3) * 3;
        let board_start_y = (y / 3) * 3;

        for i in board_start_x..(board_start_x + 3) {
            for j in board_start_y..(board_start_y + 3) {
                if self.board[i][j] == n {
                    return false;
                }
            }
        }

        true
    }

    pub fn solve(&mut self) -> bool {
        let (x, y) = match self.get_empty_cell() {
            Some(pos) => pos,
            None => return true,
        };

        for i in 1..10 {
            if self.check_if_pos_is_legal((x, y), i) {
                self.board[x][y] = i;
                if self.solve() {
                    return true;
                }
                self.board[x][y] = 0;
            }
        }

        false
    }

    pub fn print(&self) {
        for i in 0..9 {
            if i == 0 {
                println!("+-------+-------+-------+");
            }

            for j in 0..9 {
                if j == 0 {
                    print!("| ");
                }

                print!("{}", self.board[i][j]);

                if (j + 1) % 3 == 0 {
                    print!(" | ");
                }
                if (j + 1) % 3 != 0 && j != 8 {
                    print!(" ");
                }
                if j == 8 {
                    println!();
                }
            }

            if (i + 1) % 3 == 0 {
                println!("+-------+-------+-------+");
            }
        }
    }
}

fn main() {
    let board: [[u8; 9]; 9] = [
        [3, 0, 0, 0, 0, 8, 0, 0, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 8, 7, 0, 0, 0, 0, 3, 1],
        [0, 0, 3, 0, 0, 4, 0, 8, 0],
        [0, 0, 2, 8, 6, 0, 0, 0, 5],
        [0, 0, 4, 0, 9, 0, 0, 0, 0],
        [0, 3, 0, 0, 0, 0, 2, 5, 0],
        [0, 0, 0, 0, 0, 0, 0, 7, 4],
        [0, 0, 5, 2, 0, 6, 3, 0, 0],
    ];

    let mut sudoku = Sudoku::new(board);

    if sudoku.solve() {
        sudoku.print()
    } else {
        println!("Invalid grid.")
    }
}
