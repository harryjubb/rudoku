use rudoku::{Board, Sudoku};

fn main() {
    let board: Board = Board::new();
    println!("{}", board);
    println!("Rows valid: {}", board.rows_valid());
    println!("Cols valid: {}", board.cols_valid());
    println!("Valid: {}", board.board_valid());
    println!("Rows complete: {}", board.rows_complete());
    println!("Cols complete: {}", board.cols_complete());
    println!("Complete: {}", board.board_complete());
}
