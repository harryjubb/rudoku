use std::fmt;

const COMPLETED_SEGMENT_SIZE: i8 = 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9;
const WIDTH: i8 = 9;
const HEIGHT: i8 = 9;

pub struct Board {
    pub board: Vec<Vec<i8>>,
    width: i8,
    height: i8,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: vec![vec![0; WIDTH as usize]; HEIGHT as usize],
            width: WIDTH,
            height: HEIGHT,
        }
    }
}

pub trait Sudoku {
    fn rows(&self) -> Vec<Vec<i8>>;
    fn cols(&self) -> Vec<Vec<i8>>;
    fn rows_valid(&self) -> bool;
    fn cols_valid(&self) -> bool;
    fn board_valid(&self) -> bool;
    fn set_value(&mut self, i: usize, j: usize, value: i8);
}

fn segment_valid(segment: &Vec<i8>) -> bool {
    segment.iter().sum::<i8>() == COMPLETED_SEGMENT_SIZE
}

fn segments_valid(segments: Vec<Vec<i8>>) -> bool {
    segments
        .iter()
        .map(|segment| segment_valid(segment))
        .all(|segment| segment == true)
}

impl Sudoku for Board {
    fn rows(&self) -> Vec<Vec<i8>> {
        self.board.clone()
    }

    fn cols(&self) -> Vec<Vec<i8>> {
        (0..self.width)
            .map(|i| {
                self.board
                    .clone()
                    .into_iter()
                    .map(|row| row[i as usize].clone())
                    .collect::<Vec<i8>>()
            })
            .collect::<Vec<Vec<i8>>>()
    }

    fn rows_valid(&self) -> bool {
        segments_valid(self.rows())
    }

    fn cols_valid(&self) -> bool {
        segments_valid(self.cols())
    }

    fn board_valid(&self) -> bool {
        self.rows_valid() && self.cols_valid()
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
