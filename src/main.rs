use rudoku::{Board, Sudoku};

fn main() {
    let board: Board = Board::new();
    println!("{}", board);
    println!("Rows valid: {}", board.rows_valid());
}
