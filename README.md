## Sudoku Solver in Rust

A simple Sudoku solver implemented in Rust. This solver utilizes backtracking to find the solution to a given Sudoku puzzle.

> [!IMPORTANT]
> This code is not extensively tested and likely highly error-prone due to my rust-inexperience


### Example 
```rust 
use sudoku_solver::Sudoku; 

fn main() {
    // A 81 character long string representing the 9x9 Sudoku-board
    // Zeros represent empty cells
    let puzzle = "590000400008000037000286000004800209000007060037000150620900001000405000301000008";
    let mut sudoku = Sudoku::from_string(puzzle);
    
    // Attempt to solve the sudoku
    match sudoku.solve() {
        Ok(()) => println!("\nThis is the solved Sudoku-board:\n\n{}", sudoku),
        Err(()) => println!("Unable to solve board"),
    };

}

```

### Benchmark 
Running the trivial benchmark, in `bench.rs`, of 600 sudoku boards on my machine nets following result.

```bash
cargo run --release
```

```bash
Total: 368.2517ms
Average: 613.752µs
max: 24.3882ms
min 116.3µs
Successfully completed 600/600 Boards
```
