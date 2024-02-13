use crate::sudoku::Sudoku;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

pub fn sudoku_solver_benchmark() {
    let file = File::open("src/boards.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_time = Duration::new(0, 0);
    let mut completed = 0;
    let mut longest = Duration::new(0, 0);
    let mut shortest = Duration::new(10, 0);
    let mut contents: String;

    for line in reader.lines() {
        contents = line.expect("unable to read line");
        let mut sudoku = Sudoku::from_string(&contents);
        let start = Instant::now();

        sudoku.solve();
        // println!("{}", sudoku);

        let elapsed = start.elapsed();
        total_time += elapsed;

        if sudoku.is_filled() {
            completed += 1;
        }
        if elapsed > longest {
            longest = elapsed;
        } else if elapsed < shortest {
            shortest = elapsed;
        }
    }

    let avg = total_time / 600;
    println!(
        "Total: {total_time:?}\nAverage: {avg:?}\nmax: {longest:?}\nmin {shortest:?}\nSuccessfully completed {completed}/600 Boards"
    );
}
