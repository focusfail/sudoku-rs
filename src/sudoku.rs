use std::fmt;

pub struct Sudoku {
    board: [[usize; 9]; 9],
    possible: [bool; 9],
    empty_cells: Vec<(usize, usize, Vec<usize>)>,
}

impl Sudoku {
    // Creates Sudoku instance from a 2d array
    pub fn from(arr: [[usize; 9]; 9]) -> Self {
        Self {
            board: arr,
            possible: [true; 9],
            empty_cells: vec![],
        }
    }

    // Creates Sudoku instance from 81 character long string
    pub fn from_string(board_str: &str) -> Self {
        assert_eq!(board_str.len(), 81);

        let mut sudoku_board = [[0usize; 9]; 9];

        for (i, c) in board_str.chars().enumerate() {
            let row = i / 9;
            let col = i % 9;
            let digit = c.to_digit(10).expect("Invalid digit in board_str") as usize;
            sudoku_board[row][col] = digit;
        }

        Self {
            board: sudoku_board,
            possible: [true; 9],
            empty_cells: vec![],
        }
    }
    
    pub fn solve(&mut self) -> Result<(), ()> {
        self.solve_recursive();

        if self.is_filled() {
            return Ok(());
        }
        Err(())
    }

    // Solves the Sudoku puzzle, returns true if solved
    fn solve_recursive(&mut self) -> bool {
        // Identify and store empty cells
        self.get_empty_cells();

        // If there are no empty cells, the puzzle is solved
        if self.empty_cells.is_empty() {
            return true;
        }

        // Pick the first empty cell with the least possiblitie
        let smallest = &self.empty_cells[0];
        let (x, y, possibles) = smallest.to_owned();

        // Try each possible value for the cell
        for num in possibles {
            self.board[y][x] = num;

            // Recursively attempt to solve with the current value
            if self.solve_recursive() {
                return true; // If successful, return true
            }

            // If current value doesn't lead to a solution, backtrack
            self.board[y][x] = 0;
        }
        // Return false for failed path
        false
    }

    // Identify and store empty cells along with their possible values
    fn get_empty_cells(&mut self) {
        self.empty_cells.clear();

        // Iterate all cells
        for y in 0..9 {
            for x in 0..9 {
                if self.board[y][x] == 0 {
                    // Get all possible values for a cell
                    let possibles = self.possible_values(x, y);

                    // Add empty cell and possible values
                    self.empty_cells.push((x, y, possibles));
                }
            }
        }

        // For optimization, sort empty cells by amount of possible values
        self.empty_cells
            .sort_by_key(|&(_, _, ref possibles)| possibles.len());
    }

    // Identify possible values for a given cell
    fn possible_values(&mut self, x: usize, y: usize) -> Vec<usize> {
        if self.board[y][x] != 0 {
            return vec![]; // Cell is already filled
        }
        self.possible = [true; 9];

        let end = (x / 3) * 3;
        let start = (y / 3) * 3;

        for ind in 0..9 {
            let i = ind / 3;
            let j = ind % 3;

            // ind'th cell in according sudoku block of the cell
            let block_cell = self.board[start + i][end + j];

            // ind'th cell in the according column of the cell
            let col_cell = self.board[ind][x];

            // ind'th cell in the according row of the cell
            let row_cell = self.board[y][ind];

            // Update possible values based on the existing numbers in the block, row, and column
            // If a number occurs in the 'block_cell', 'col_cell' or 'row_cell'
            // it is not a possible value
            if block_cell > 0 && self.possible[block_cell - 1] {
                self.possible[block_cell - 1] = false;
            }

            if col_cell > 0 && self.possible[col_cell - 1] {
                self.possible[col_cell - 1] = false;
            }

            if row_cell > 0 && self.possible[row_cell - 1] {
                self.possible[row_cell - 1] = false;
            }
        }

        // Convert array of booleans to vec of possible.
        // INFO: I dont remember why I took the bool-array approach in the first place 👍
        self.possible
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| if p { Some(i + 1) } else { None })
            .collect()
    }

    // Check if the Sudoku puzzle is completely filled
    pub fn is_filled(&self) -> bool {
        for i in 0..9 {
            if self.board[i].contains(&0usize) {
                return false; // If any row contains an empty cell, puzzle is not filled
            }
        }
        true
    }
}

// Implement Display trait for pretty printing Sudoku
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
