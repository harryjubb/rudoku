use std::fmt;

const COMPLETED_SEGMENT_SIZE: i32 = 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9;
const WIDTH: usize = 9;
const HEIGHT: usize = 9;
const SQUARE_SIZE: usize = 3;

fn segment_valid(segment: &Vec<i32>) -> bool {
    let segment_copy = segment.clone();
    let segment_no_zeroes = segment_copy
        .iter()
        .filter(|x| (**x) > 0)
        .collect::<Vec<&i32>>();
    let mut segment_no_zeroes_dedup = segment_no_zeroes.clone();
    segment_no_zeroes_dedup.sort();
    segment_no_zeroes_dedup.dedup();
    segment_no_zeroes.len() == segment_no_zeroes_dedup.len()
}

fn segments_valid(segments: Vec<Vec<i32>>) -> bool {
    segments
        .iter()
        .map(|segment| segment_valid(segment))
        .all(|segment| segment == true)
}

fn segment_complete(segment: &Vec<i32>) -> bool {
    segment.iter().sum::<i32>() == COMPLETED_SEGMENT_SIZE
}

fn segments_complete(segments: Vec<Vec<i32>>) -> bool {
    segments
        .iter()
        .map(|segment| segment_complete(segment))
        .all(|segment| segment == true)
}

pub struct Board {
    pub board: Vec<Vec<i32>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: vec![vec![0; WIDTH]; HEIGHT],
        }
    }
    pub fn from_string(board_string: &str) -> Self {
        Self {
            board: board_string
                .chars()
                .map(|character| character.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
                .chunks(WIDTH)
                .map(|chunk| chunk.to_vec())
                .collect(),
        }
    }
}

pub trait Sudoku {
    fn rows(&self) -> Vec<Vec<i32>>;
    fn cols(&self) -> Vec<Vec<i32>>;
    fn squares(&self) -> Vec<Vec<i32>>;
    fn rows_valid(&self) -> bool;
    fn cols_valid(&self) -> bool;
    fn squares_valid(&self) -> bool;
    fn board_valid(&self) -> bool;
    fn rows_complete(&self) -> bool;
    fn cols_complete(&self) -> bool;
    fn squares_complete(&self) -> bool;
    fn board_complete(&self) -> bool;
    fn set_value(&mut self, i: usize, j: usize, value: i32);
}

impl Sudoku for Board {
    fn rows(&self) -> Vec<Vec<i32>> {
        self.board.clone()
    }

    fn cols(&self) -> Vec<Vec<i32>> {
        (0..WIDTH)
            .map(|i| {
                self.board
                    .clone()
                    .into_iter()
                    .map(|row| row[i].clone())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
    }

    fn squares(&self) -> Vec<Vec<i32>> {
        // Chunk the numbers within rows into groups of the square size
        let chunked_within_rows = self
            .board
            .iter()
            .map(|row| {
                row.chunks(SQUARE_SIZE)
                    .map(|chunk| chunk.to_vec())
                    .collect::<Vec<Vec<i32>>>()
            })
            .collect::<Vec<Vec<Vec<i32>>>>();
        // Chunk the rows themselves into chunks of the square size
        let chunked_rows = chunked_within_rows
            .chunks(SQUARE_SIZE)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<Vec<Vec<i32>>>>>();
        // Get each square and concatenate them
        let squares = (0..(WIDTH / SQUARE_SIZE))
            .map(|i| {
                (0..(HEIGHT / SQUARE_SIZE))
                    .map(|j| (&chunked_rows[i][j]).clone())
                    .collect::<Vec<Vec<Vec<i32>>>>()
                    .concat()
            })
            .collect::<Vec<Vec<Vec<i32>>>>()
            .concat();

        squares
    }

    fn rows_valid(&self) -> bool {
        segments_valid(self.rows())
    }

    fn cols_valid(&self) -> bool {
        segments_valid(self.cols())
    }

    fn squares_valid(&self) -> bool {
        segments_valid(self.squares())
    }

    fn board_valid(&self) -> bool {
        self.rows_valid() && self.cols_valid() && self.squares_valid()
    }

    fn rows_complete(&self) -> bool {
        segments_complete(self.rows())
    }

    fn cols_complete(&self) -> bool {
        segments_complete(self.cols())
    }

    fn squares_complete(&self) -> bool {
        segments_complete(self.squares())
    }

    fn board_complete(&self) -> bool {
        self.rows_complete() && self.cols_complete() && self.squares_complete()
    }

    fn set_value(&mut self, i: usize, j: usize, value: i32) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_board_from_new() {
        let board = Board::new();
        assert_eq!(board.board, vec![vec![0; 9]; 9]);
    }
    #[test]
    fn blank_board_is_valid() {
        let board = Board::new();
        assert_eq!(board.board_valid(), true);
        assert_eq!(board.board_complete(), false);
    }

    #[test]
    fn correct_board_from_string() {
        let board = Board::from_string(
            "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
        );
        assert_eq!(
            board.board,
            [
                [3, 7, 9, 0, 0, 0, 0, 1, 4],
                [0, 6, 0, 0, 1, 0, 0, 7, 0],
                [0, 8, 0, 0, 0, 9, 0, 0, 5],
                [4, 3, 5, 0, 0, 7, 0, 0, 0],
                [0, 9, 0, 0, 4, 0, 0, 2, 0],
                [0, 0, 0, 8, 0, 0, 4, 3, 6],
                [9, 0, 0, 7, 0, 0, 0, 8, 0],
                [0, 4, 0, 0, 8, 0, 0, 5, 0],
                [8, 5, 0, 0, 0, 0, 2, 4, 9]
            ]
        );
    }

    #[test]
    fn incomplete_valid_board_string_is_valid() {
        let board = Board::from_string(
            "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
        );
        assert_eq!(board.board_valid(), true);
        assert_eq!(board.board_complete(), false);
    }
}
