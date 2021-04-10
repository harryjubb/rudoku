use std::fmt;

pub struct Board {
    board: Vec<Vec<i8>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: vec![vec![0; 9]; 9],
        }
    }
}

trait Sudoku {
    // fn rows(&self) -> Vec<Vec<i8>>;
    // fn cols(&self) -> [[i8; 9]; 9];
    fn valid(&self) -> bool;
    fn set_value(&mut self, i: usize, j: usize, value: i8);
}

impl Sudoku for Board {
    // fn rows(&self) {
    //     self.board
    // }

    // fn cols(&self) -> [[i8; 9]; 9] {
    //     // Vec
    // }

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
