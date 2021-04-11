use std::fmt;

const COMPLETED_SEGMENT_SIZE: i8 = 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9;
const WIDTH: i8 = 9;
const HEIGHT: i8 = 9;
const SQUARE_SIZE: i8 = 3;

fn segment_valid(segment: &Vec<i8>) -> bool {
    let segment_copy = segment.clone();
    let segment_no_zeroes = segment_copy
        .iter()
        .filter(|x| (**x) > 0)
        .collect::<Vec<&i8>>();
    let mut segment_no_zeroes_dedup = segment.clone();
    segment_no_zeroes_dedup.sort();
    segment_no_zeroes_dedup.dedup();
    segment_no_zeroes.len() == segment_no_zeroes_dedup.len()
}

fn segments_valid(segments: Vec<Vec<i8>>) -> bool {
    segments
        .iter()
        .map(|segment| segment_valid(segment))
        .all(|segment| segment == true)
}

fn segment_complete(segment: &Vec<i8>) -> bool {
    segment.iter().sum::<i8>() == COMPLETED_SEGMENT_SIZE
}

fn segments_complete(segments: Vec<Vec<i8>>) -> bool {
    segments
        .iter()
        .map(|segment| segment_complete(segment))
        .all(|segment| segment == true)
}

pub struct Board {
    pub board: Vec<Vec<i8>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: vec![vec![0; WIDTH as usize]; HEIGHT as usize],
        }
    }
}

pub trait Sudoku {
    fn rows(&self) -> Vec<Vec<i8>>;
    fn cols(&self) -> Vec<Vec<i8>>;
    // fn squares(&self) -> Vec<Vec<i8>>;
    fn rows_valid(&self) -> bool;
    fn cols_valid(&self) -> bool;
    fn board_valid(&self) -> bool;
    fn rows_complete(&self) -> bool;
    fn cols_complete(&self) -> bool;
    fn board_complete(&self) -> bool;
    fn set_value(&mut self, i: usize, j: usize, value: i8);
}

impl Sudoku for Board {
    fn rows(&self) -> Vec<Vec<i8>> {
        self.board.clone()
    }

    fn cols(&self) -> Vec<Vec<i8>> {
        (0..WIDTH)
            .map(|i| {
                self.board
                    .clone()
                    .into_iter()
                    .map(|row| row[i as usize].clone())
                    .collect::<Vec<i8>>()
            })
            .collect::<Vec<Vec<i8>>>()
    }

    // fn squares(&self) -> Vec<Vec<i8>> {
    //     (0..WIDTH).step_by(SQUARE_SIZE)
    // }

    fn rows_valid(&self) -> bool {
        segments_valid(self.rows())
    }

    fn cols_valid(&self) -> bool {
        segments_valid(self.cols())
    }

    fn board_valid(&self) -> bool {
        self.rows_valid() && self.cols_valid()
    }

    fn rows_complete(&self) -> bool {
        segments_complete(self.rows())
    }

    fn cols_complete(&self) -> bool {
        segments_complete(self.cols())
    }

    fn board_complete(&self) -> bool {
        self.rows_complete() && self.cols_complete()
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
