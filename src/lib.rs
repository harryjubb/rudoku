use std::collections::HashMap;
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
    // pub possible_values: HashMap<(usize, usize), Vec<i32>>,
}

impl Board {
    pub fn new() -> Self {
        // let mut possible_values = HashMap::new();
        // (0..HEIGHT).for_each(|i| {
        //     (0..WIDTH).for_each(|j| {
        //         possible_values.insert((i, j), ALL_POSSIBLE_VALUES);
        //     });
        // });
        Self {
            board: vec![vec![0; WIDTH]; HEIGHT],
            // possible_values: possible_values,
        }
    }
    pub fn from_string(board_string: &str) -> Self {
        let board = board_string
            .chars()
            .map(|character| character.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>()
            .chunks(WIDTH)
            .map(|chunk| chunk.to_vec())
            .collect();
        // let possible_values =
        Self { board: board }
    }
}

pub trait Sudoku {
    fn rows(&self) -> Vec<Vec<i32>>;
    fn cols(&self) -> Vec<Vec<i32>>;
    fn squares(&self) -> Vec<Vec<i32>>;
    fn get_row(&self, i: usize) -> Vec<i32>;
    fn get_col(&self, j: usize) -> Vec<i32>;
    fn get_square(&self, i: usize, j: usize) -> Vec<i32>;
    fn rows_valid(&self) -> bool;
    fn cols_valid(&self) -> bool;
    fn squares_valid(&self) -> bool;
    fn board_valid(&self) -> bool;
    fn rows_complete(&self) -> bool;
    fn cols_complete(&self) -> bool;
    fn squares_complete(&self) -> bool;
    fn board_complete(&self) -> bool;
    fn set_value(&mut self, i: usize, j: usize, value: i32);
    fn possible_values(&self) -> HashMap<(usize, usize), Vec<i32>>;
    fn solve_tick(&mut self) -> i32;
    fn solve(&mut self) -> i32;
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
        // Chunk rows into vertical SQUARE_SIZE
        let chunked_rows = self
            .board
            .chunks(SQUARE_SIZE)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<Vec<i32>>>>();

        // Chunk the rows into horizontal SQUARE_SIZE
        let chunked_squares = chunked_rows
            .iter()
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|row| {
                        row.chunks(SQUARE_SIZE)
                            .map(|chunk| chunk.to_vec())
                            .collect::<Vec<Vec<i32>>>()
                    })
                    .collect::<Vec<Vec<Vec<i32>>>>()
            })
            .collect::<Vec<Vec<Vec<Vec<i32>>>>>();

        // Get vectors of the numbers in each square
        let squares = chunked_squares
            .iter()
            .map(|square_row| {
                (0..SQUARE_SIZE)
                    .map(|i| {
                        square_row
                            .iter()
                            .map(|chunked_row| chunked_row.iter().nth(i).unwrap().clone())
                            .collect::<Vec<Vec<i32>>>()
                            .concat()
                    })
                    .collect::<Vec<Vec<i32>>>()
            })
            .collect::<Vec<Vec<Vec<i32>>>>()
            .concat();

        squares
    }

    fn get_row(&self, row_index: usize) -> Vec<i32> {
        self.rows()[row_index].clone()
    }

    fn get_col(&self, col_index: usize) -> Vec<i32> {
        self.cols()[col_index].clone()
    }

    fn get_square(&self, row_index: usize, col_index: usize) -> Vec<i32> {
        self.squares()
            .chunks(SQUARE_SIZE)
            .nth(row_index / SQUARE_SIZE)
            .unwrap()
            .iter()
            .nth(col_index / SQUARE_SIZE)
            .unwrap()
            .clone()
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

    fn set_value(&mut self, row_index: usize, col_index: usize, value: i32) {
        self.board[row_index][col_index] = value
    }

    fn possible_values(&self) -> HashMap<(usize, usize), Vec<i32>> {
        let mut possible_values: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
        (0..HEIGHT).for_each(|row_index| {
            (0..WIDTH).for_each(|col_index| {
                let current_value = self
                    .board
                    .iter()
                    .nth(row_index)
                    .unwrap()
                    .iter()
                    .nth(col_index)
                    .unwrap();
                let key = (row_index, col_index);
                let value = match current_value {
                    x if x > &0 => vec![*x],
                    _ => vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
                        .into_iter()
                        .filter(|number| {
                            (self.get_row(row_index).iter().all(|i| i != number))
                                && (self.get_col(col_index).iter().all(|i| i != number))
                                && (self
                                    .get_square(row_index, col_index)
                                    .iter()
                                    .all(|i| i != number))
                        })
                        .collect(),
                };
                possible_values.insert(key, value);
            });
        });
        possible_values
    }

    fn solve_tick(&mut self) -> i32 {
        // For values with only one possible value, fill them in
        let mut values_set = 0;
        let possible_values = self.possible_values();
        let keys = possible_values.keys();
        keys.for_each(|key| {
            let possible = possible_values.get(&key);
            match possible {
                Some(possible) => {
                    if possible.len() == 1 && self.board[key.0][key.1] == 0 {
                        self.set_value(key.0, key.1, possible[0]);
                        values_set += 1;
                        println!("set a value")
                    }
                }
                None => panic!("Tried to solve an invalid cell"),
            }
        });
        values_set
    }

    fn solve(&mut self) -> i32 {
        let mut steps_taken = 0;
        while !self.board_complete() {
            let values_set = self.solve_tick();
            steps_taken += 1;
            if values_set == 0 {
                println!("Failed board state");
                println!("{:?}", self.board);
                panic!("Could not solve: ran out of definitive possible values")
            }
        }
        steps_taken
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
    fn set_value() {
        let mut board = Board::new();
        board.set_value(0, 0, 1);
        assert_eq!(
            board.board,
            [
                [1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ]
        );
        board.set_value(3, 5, 9);
        assert_eq!(
            board.board,
            [
                [1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 9, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0],
            ]
        );
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

    #[test]
    fn complete_valid_board_string_is_valid_and_complete() {
        let board = Board::from_string(
            "845632179732918654196745328683574912457291836219863547361429785574186293928357461",
        );
        assert_eq!(board.board_valid(), true);
        assert_eq!(board.rows_complete(), true);
        assert_eq!(board.cols_complete(), true);
        assert_eq!(board.squares_complete(), true);
        assert_eq!(board.board_complete(), true);
    }

    #[test]
    fn test_squares() {
        let board = Board::from_string(
            "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
        );
        assert_eq!(
            board.squares(),
            vec![
                vec![3, 7, 9, 0, 6, 0, 0, 8, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 9],
                vec![0, 1, 4, 0, 7, 0, 0, 0, 5],
                vec![4, 3, 5, 0, 9, 0, 0, 0, 0],
                vec![0, 0, 7, 0, 4, 0, 8, 0, 0],
                vec![0, 0, 0, 0, 2, 0, 4, 3, 6],
                vec![9, 0, 0, 0, 4, 0, 8, 5, 0],
                vec![7, 0, 0, 0, 8, 0, 0, 0, 0],
                vec![0, 8, 0, 0, 5, 0, 2, 4, 9]
            ]
        );
    }

    #[test]
    fn test_get_row() {
        let board = Board::from_string(
            "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
        );
        assert_eq!(board.get_row(0), vec![3, 7, 9, 0, 0, 0, 0, 1, 4]);
        assert_eq!(board.get_row(1), vec![0, 6, 0, 0, 1, 0, 0, 7, 0]);
        assert_eq!(board.get_row(2), vec![0, 8, 0, 0, 0, 9, 0, 0, 5]);
        assert_eq!(board.get_row(3), vec![4, 3, 5, 0, 0, 7, 0, 0, 0]);
        assert_eq!(board.get_row(4), vec![0, 9, 0, 0, 4, 0, 0, 2, 0]);
        assert_eq!(board.get_row(5), vec![0, 0, 0, 8, 0, 0, 4, 3, 6]);
        assert_eq!(board.get_row(6), vec![9, 0, 0, 7, 0, 0, 0, 8, 0]);
        assert_eq!(board.get_row(7), vec![0, 4, 0, 0, 8, 0, 0, 5, 0]);
        assert_eq!(board.get_row(8), vec![8, 5, 0, 0, 0, 0, 2, 4, 9]);
    }

    #[test]
    fn test_get_col() {
        let board = Board::from_string(
            "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
        );
        assert_eq!(board.get_col(0), vec![3, 0, 0, 4, 0, 0, 9, 0, 8]);
        assert_eq!(board.get_col(1), vec![7, 6, 8, 3, 9, 0, 0, 4, 5]);
        assert_eq!(board.get_col(2), vec![9, 0, 0, 5, 0, 0, 0, 0, 0]);
        assert_eq!(board.get_col(3), vec![0, 0, 0, 0, 0, 8, 7, 0, 0]);
        assert_eq!(board.get_col(4), vec![0, 1, 0, 0, 4, 0, 0, 8, 0]);
        assert_eq!(board.get_col(5), vec![0, 0, 9, 7, 0, 0, 0, 0, 0]);
        assert_eq!(board.get_col(6), vec![0, 0, 0, 0, 0, 4, 0, 0, 2]);
        assert_eq!(board.get_col(7), vec![1, 7, 0, 0, 2, 3, 8, 5, 4]);
        assert_eq!(board.get_col(8), vec![4, 0, 5, 0, 0, 6, 0, 0, 9]);
    }

    #[test]
    fn test_get_square() {
        let board = Board::from_string(
            "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
        );
        assert_eq!(board.get_square(0, 0), vec![3, 7, 9, 0, 6, 0, 0, 8, 0]);
        assert_eq!(board.get_square(8, 8), vec![0, 8, 0, 0, 5, 0, 2, 4, 9]);
        assert_eq!(board.get_square(4, 4), vec![0, 0, 7, 0, 4, 0, 8, 0, 0]);
        assert_eq!(board.get_square(4, 2), vec![4, 3, 5, 0, 9, 0, 0, 0, 0]);
        assert_eq!(board.get_square(7, 3), vec![7, 0, 0, 0, 8, 0, 0, 0, 0]);
    }

    #[test]
    fn test_possible_values() {
        let board = Board::from_string(
            "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
        );
        assert_eq!(board.possible_values().get(&(0, 0)), Some(&vec![3]));
        assert_eq!(board.possible_values().get(&(1, 0)), Some(&vec![2, 5]));
        assert_eq!(board.possible_values().get(&(0, 3)), Some(&vec![2, 5, 6]));
    }

    #[test]
    fn test_solve_tick() {
        let mut board = Board::from_string(
            "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
        );
        assert_eq!(board.solve_tick(), 2);
        assert_eq!(
            board.board,
            [
                [3, 7, 9, 0, 0, 0, 0, 1, 4],
                [0, 6, 0, 0, 1, 0, 0, 7, 0],
                [0, 8, 0, 0, 0, 9, 0, 6, 5],
                [4, 3, 5, 0, 0, 7, 0, 9, 0],
                [0, 9, 0, 0, 4, 0, 0, 2, 0],
                [0, 0, 0, 8, 0, 0, 4, 3, 6],
                [9, 0, 0, 7, 0, 0, 0, 8, 0],
                [0, 4, 0, 0, 8, 0, 0, 5, 0],
                [8, 5, 0, 0, 0, 0, 2, 4, 9],
            ]
        );
        assert_eq!(board.solve_tick(), 2);
        assert_eq!(
            board.board,
            [
                [3, 7, 9, 0, 0, 0, 8, 1, 4],
                [0, 6, 0, 0, 1, 0, 0, 7, 0],
                [0, 8, 0, 0, 0, 9, 3, 6, 5],
                [4, 3, 5, 0, 0, 7, 0, 9, 0],
                [0, 9, 0, 0, 4, 0, 0, 2, 0],
                [0, 0, 0, 8, 0, 0, 4, 3, 6],
                [9, 0, 0, 7, 0, 0, 0, 8, 0],
                [0, 4, 0, 0, 8, 0, 0, 5, 0],
                [8, 5, 0, 0, 0, 0, 2, 4, 9]
            ]
        );
        assert_eq!(board.solve_tick(), 3);
        assert_eq!(
            board.board,
            [
                [3, 7, 9, 0, 0, 0, 8, 1, 4],
                [0, 6, 0, 0, 1, 0, 9, 7, 2],
                [0, 8, 0, 0, 0, 9, 3, 6, 5],
                [4, 3, 5, 0, 0, 7, 1, 9, 0],
                [0, 9, 0, 0, 4, 0, 0, 2, 0],
                [0, 0, 0, 8, 0, 0, 4, 3, 6],
                [9, 0, 0, 7, 0, 0, 0, 8, 0],
                [0, 4, 0, 0, 8, 0, 0, 5, 0],
                [8, 5, 0, 0, 0, 0, 2, 4, 9],
            ]
        );
        assert_eq!(board.solve_tick(), 4);
        assert_eq!(
            board.board,
            [
                [3, 7, 9, 0, 0, 0, 8, 1, 4],
                [5, 6, 4, 0, 1, 0, 9, 7, 2],
                [0, 8, 0, 0, 0, 9, 3, 6, 5],
                [4, 3, 5, 0, 0, 7, 1, 9, 8],
                [0, 9, 0, 0, 4, 0, 0, 2, 0],
                [0, 0, 0, 8, 0, 0, 4, 3, 6],
                [9, 0, 0, 7, 0, 0, 6, 8, 0],
                [0, 4, 0, 0, 8, 0, 0, 5, 0],
                [8, 5, 0, 0, 0, 0, 2, 4, 9]
            ]
        );
        assert_eq!(board.solve_tick(), 3);
        assert_eq!(
            board.board,
            [
                [3, 7, 9, 0, 0, 0, 8, 1, 4],
                [5, 6, 4, 3, 1, 0, 9, 7, 2],
                [0, 8, 0, 0, 0, 9, 3, 6, 5],
                [4, 3, 5, 0, 0, 7, 1, 9, 8],
                [0, 9, 0, 0, 4, 0, 0, 2, 7],
                [0, 0, 0, 8, 0, 0, 4, 3, 6],
                [9, 0, 0, 7, 0, 0, 6, 8, 0],
                [0, 4, 0, 0, 8, 0, 7, 5, 0],
                [8, 5, 0, 0, 0, 0, 2, 4, 9]
            ]
        );
    }

    #[test]
    fn test_solve_easy() {
        let mut board = Board::from_string(
            "002000500010705020400090007049000730801030409036000210200080004080902060007000800",
        );
        assert_eq!(board.solve(), 12);
        assert_eq!(
            board.board,
            [
                [9, 7, 2, 8, 6, 3, 5, 4, 1],
                [6, 1, 8, 7, 4, 5, 9, 2, 3],
                [4, 5, 3, 2, 9, 1, 6, 8, 7],
                [5, 4, 9, 1, 2, 8, 7, 3, 6],
                [8, 2, 1, 6, 3, 7, 4, 5, 9],
                [7, 3, 6, 4, 5, 9, 2, 1, 8],
                [2, 9, 5, 3, 8, 6, 1, 7, 4],
                [1, 8, 4, 9, 7, 2, 3, 6, 5],
                [3, 6, 7, 5, 1, 4, 8, 9, 2]
            ]
        );
        assert_eq!(board.board_valid(), true);
        assert_eq!(board.board_complete(), true);
    }

    // #[test]
    // fn test_solve() {
    //     let mut board = Board::from_string(
    //         "379000014060010070080009005435007000090040020000800436900700080040080050850000249",
    //     );
    //     assert_eq!(board.solve(), 20);
    //     assert_eq!(
    //         board.board,
    //         [
    //             [3, 7, 9, 0, 0, 0, 0, 1, 4],
    //             [0, 6, 0, 0, 1, 0, 0, 7, 0],
    //             [0, 8, 0, 0, 0, 9, 0, 6, 5],
    //             [4, 3, 5, 0, 0, 7, 0, 9, 0],
    //             [0, 9, 0, 0, 4, 0, 0, 2, 0],
    //             [0, 0, 0, 8, 0, 0, 4, 3, 6],
    //             [9, 0, 0, 7, 0, 0, 0, 8, 0],
    //             [0, 4, 0, 0, 8, 0, 0, 5, 0],
    //             [8, 5, 0, 0, 0, 0, 2, 4, 9],
    //         ]
    //     );
    // }
}
