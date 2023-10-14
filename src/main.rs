use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};
use sudoku::Sudoku;

fn board_from_string(board: String) -> [[usize; 9]; 9] {
    assert_eq!(board.len(), 81);

    let mut sudoku_board = [[0usize; 9]; 9];

    for (i, c) in board.chars().enumerate() {
        let row = i / 9;
        let col = i % 9;
        let digit = c.to_digit(10).expect("Invalid digit in input") as usize;
        sudoku_board[row][col] = digit;
    }

    sudoku_board
}

fn main() {
    let file = File::open("src/boards.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_time = Duration::new(0, 0);
    let count_boards = 600;
    let mut completed = 0;
    let mut longest = Duration::new(0, 0);
    let mut shortest = Duration::new(10, 0);
    let mut contents: String;
    let mut longest_board: Sudoku = Sudoku::from([[0usize; 9]; 9]);

    for (i, line) in reader.lines().enumerate() {
        contents = line.expect("unable to read line");
        let board = board_from_string(contents);
        let mut sudoku = Sudoku::from(board);
        let start = Instant::now();

        sudoku.solve();

        let elapsed = start.elapsed();
        println!("\n\n{}", sudoku);
        println!("{:>13?}\n", i + 1);
        total_time += elapsed;

        if sudoku.is_filled() {
            completed += 1;
        }
        if elapsed > longest {
            longest = elapsed;
            longest_board = Sudoku::from(board);
        } else if elapsed < shortest {
            shortest = elapsed;
        }
    }
    let avg = total_time / count_boards;
    println!(
        "Tot: {total_time:?} | Avg: {avg:?} | max: {longest:?} | min {shortest:?} | Successfully completed {completed}/{count_boards} Boards"
    );
    println!("{}", longest_board);
}
