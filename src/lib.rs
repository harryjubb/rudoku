use std::fmt;

const COMPLETED_SEGMENT: i8 = 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9;

pub struct Board {
    pub board: Vec<Vec<i8>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: vec![vec![0; 9]; 9],
        }
    }
}

pub trait Sudoku {
    fn rows(&self) -> &[Vec<i8>];
    // fn cols(&self) -> [[i8; 9]; 9];
    fn rows_valid(&self) -> bool;
    // fn cols_valid(&self) -> bool;
    fn valid(&self) -> bool;
    fn set_value(&mut self, i: usize, j: usize, value: i8);
}

impl Sudoku for Board {
    fn rows(&self) -> &[Vec<i8>] {
        self.board.as_slice()
    }

    // fn cols(&self) -> [[i8; 9]; 9] {
    //     // Vec
    // }

    fn rows_valid(&self) -> bool {
        self.rows()
            .iter()
            .map(|row| row.iter().sum::<i8>() == COMPLETED_SEGMENT)
            .all(|row| row == true)
    }

    fn valid(&self) -> bool {
        false
    }

    fn set_value(&mut self, i: usize, j: usize, value: i8) {
        self.board[i][j] = value
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.board
                .iter()
                .map(|row| row
                    .iter()
                    .map(|col| col.to_string())
                    .collect::<Vec<String>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
