use std::fmt;

pub struct Sudoku {
    board: [[usize; 9]; 9],
    possible: [bool; 9],
    empty_cells: Vec<(usize, usize, Vec<usize>)>,
}

impl Sudoku {
    pub fn from(arr: [[usize; 9]; 9]) -> Sudoku {
        Sudoku {
            board: arr,
            possible: [true; 9],
            empty_cells: vec![],
        }
    }

    pub fn solve(&mut self) -> bool {
        self.find_empty_cells();
        if self.empty_cells.is_empty() {
            return true;
        }

        let smallest = &self.empty_cells[0];
        let (x, y, possibles) = smallest.to_owned();

        for num in possibles {
            self.board[y][x] = num;

            if self.solve() {
                return true;
            }
            self.board[y][x] = 0;
        }
        false
    }

    fn find_empty_cells(&mut self) {
        self.empty_cells.clear();
        for y in 0..9 {
            for x in 0..9 {
                if self.board[y][x] == 0 {
                    let possibles = self.possible_values(x, y);
                    self.empty_cells.push((x, y, possibles));
                }
            }
        }

        self.empty_cells
            .sort_by_key(|&(_, _, ref possibles)| possibles.len());
    }

    fn possible_values(&mut self, x: usize, y: usize) -> Vec<usize> {
        if self.board[y][x] != 0 {
            return vec![];
        }
        self.possible = [true; 9];

        let end = (x / 3) * 3;
        let start = (y / 3) * 3;

        for ind in 0..9 {
            let i = ind / 3;
            let j = ind % 3;

            let block_cell = self.board[start + i][end + j];
            let col_cell = self.board[ind][x];
            let row = self.board[y][ind];

            if block_cell != 0 {
                self.possible[block_cell - 1] = false;
            }

            if col_cell != 0 {
                self.possible[col_cell - 1] = false;
            }

            if row != 0 {
                self.possible[row - 1] = false;
            }
        }

        self.possible
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| if p { Some(i + 1) } else { None })
            .collect()
    }

    pub fn is_filled(&self) -> bool {
        for i in 0..9 {
            if self.board[i].contains(&0usize) {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sudoku_board = String::new();

        for (i, row) in self.board.iter().enumerate() {
            sudoku_board.push(' ');
            for (j, cell) in row.iter().enumerate() {
                if cell == &0usize {
                    sudoku_board.push_str(". ");
                } else {
                    let value = format!("{cell} ");
                    sudoku_board.push_str(&value);
                }

                if (j + 1) % 3 == 0 && j != 8 {
                    sudoku_board.push_str("| ");
                }
            }
            if (i + 1) % 3 == 0 && i != 0 && i != 8 {
                sudoku_board.push_str("\n-------+-------+-------\n")
            } else {
                sudoku_board.push('\n');
            }
        }

        write!(f, "{}", sudoku_board)
    }
}
