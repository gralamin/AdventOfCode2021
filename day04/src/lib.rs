extern crate boardlib;
extern crate filelib;
use crate::boardlib::BoardTraversable;
use std::collections::HashSet;
use std::collections::HashMap;

pub use filelib::load;
pub use filelib::parse_csv_i32_lines;
pub use filelib::split_lines_by_blanks;

trait SolvableBingoBoard {
    fn has_bingo(&self) -> bool;
    fn score(&self) -> u32;
    fn mark_number(&mut self, num: u32);
}

#[derive(Debug)]
struct BingoBoard {
    board: boardlib::Board<u32>,

    /* Game state variables
     *
     * We currently need to know which numbers are marked only.
     */
    called_numbers: HashSet<u32>,
}

impl BingoBoard {
    fn new(width: usize, height: usize, values: Vec<i32>) -> BingoBoard {
        let converted: Vec<u32> = values.iter().map(|x| (*x).try_into().unwrap()).collect();
        return BingoBoard {
            board: boardlib::Board::new(width, height, converted),
            called_numbers: HashSet::new(),
        };
    }
}

impl SolvableBingoBoard for BingoBoard {
    /// Have bingo if all rows or columns are marked
    ///
    /// Worst case O(n) (no numbers marked, read entire board twice)
    /// Assuming lookup of numbers in my set_hash is O(1)
    fn has_bingo(&self) -> bool {
        // columns: on a given x go down all the ys
        let mut columns: HashMap<usize, usize> = HashMap::new();
        let mut rows: HashMap<usize, usize> = HashMap::new();
        for cur_coord in self.board.coord_iter() {
            if let Some(cur_num) = self.board.get_value(cur_coord) {
                if self.called_numbers.contains(&cur_num) {
                    let column_counter = columns.entry(cur_coord.x).or_insert(0);
                    *column_counter += 1;
                    let row_counter = rows.entry(cur_coord.y).or_insert(0);
                    *row_counter += 1;
                }
            }
        }
        for (_, value) in rows {
            if value == self.board.get_width() {
                return true;
            }
        }
        for (_, value) in columns {
            if value == self.board.get_height() {
                return true;
            }
        }

        return false;
    }

    /// Score is sum of all unmarked numbers
    ///
    /// O(n) by nature of iterating over the board.
    /// Assuming lookup of numbers in my set_hash is O(1)
    fn score(&self) -> u32 {
        let mut total: u32 = 0;
        for y in 0..self.board.get_height() {
            for x in 0..self.board.get_width() {
                if let Some(cur_num) = self.board.get_value(boardlib::BoardCoordinate::new(x, y)) {
                    if !self.called_numbers.contains(&cur_num) {
                        total += cur_num;
                    }
                }
            }
        }
        return total;
    }

    fn mark_number(&mut self, num: u32) {
        self.called_numbers.insert(num);
    }
}

/// Iterate through the numbers for the solution
///
/// This could potentially be a bit slow on large inputs, but not optimizing unless I need to.
/// I also don't consider if two boards can win at once, in which case I probably want the highest scoring one
///
/// ```
/// let numbers = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
/// let boards = vec![vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19],
///                   vec![3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6],
///                   vec![14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,]];
/// assert_eq!(day04::puzzle_a(&numbers, &boards), 4512);
/// ```
pub fn puzzle_a(called_numbers: &Vec<i32>, boards: &Vec<Vec<i32>>) -> u32 {
    // Theoretically we could skip to the first 4 numbers called, as there cannot be a bingo until the
    // fifth number, but not going to bother optimizing that.
    let mut board_vec: Vec<BingoBoard> = boards
        .iter()
        .map(|nums| BingoBoard::new(5, 5, nums.to_vec()))
        .collect();

    for calling_number in called_numbers.iter() {
        // iterate by index so we can mutate structure safely
        let as_u32: u32 = (*calling_number).try_into().unwrap();
        for i in 0..board_vec.len() {
            board_vec[i].mark_number(as_u32);
            if board_vec[i].has_bingo() {
                return board_vec[i].score() * as_u32;
            }
        }
    }
    // If we don't get a bingo throw a number I know is wrong
    return 0;
}

/// Iterate through the numbers for the last board to win.
///
/// This could potentially be a bit slow on large inputs, but not optimizing unless I need to.
/// I also don't consider if two boards tie for last, in which case based on the prompt I would want the lowest score.
///
/// ```
/// let numbers = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
/// let boards = vec![vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19],
///                   vec![3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6],
///                   vec![14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,]];
/// assert_eq!(day04::puzzle_b(&numbers, &boards), 1924);
/// ```
pub fn puzzle_b(called_numbers: &Vec<i32>, boards: &Vec<Vec<i32>>) -> u32 {
    // Theoretically we could skip to the first 4 numbers called, as there cannot be a bingo until the
    // fifth number, but not going to bother optimizing that.
    let mut board_vec: Vec<BingoBoard> = boards
        .iter()
        .map(|nums| BingoBoard::new(5, 5, nums.to_vec()))
        .collect();

    let mut index_bingoed: HashSet<usize> = HashSet::new();
    for calling_number in called_numbers.iter() {
        let as_u32: u32 = (*calling_number).try_into().unwrap();
        // iterate by index so we can mutate structure safely
        for i in 0..board_vec.len() {
            if index_bingoed.contains(&i) {
                // already bingoed, ignore it.
                continue;
            }
            board_vec[i].mark_number(as_u32);
            if board_vec[i].has_bingo() {
                index_bingoed.insert(i);
                if index_bingoed.len() == boards.len() {
                    return board_vec[i].score() * as_u32;
                }
            }
        }
    }
    // If we don't get a bingo throw a number I know is wrong
    return 0;
}

/// Input parsing, split all the numbers into boards.
///
/// This remains here as it seems fairly specific to this input
///
/// ```
/// let ins = vec![
///   vec!["22 13 17 11  0", " 8  2 23  4 24", "21  9 14 16  7", " 6 10  3 18  5", " 1 12 20 15 19"].iter().map(|s| s.to_string()).collect(),
///   vec![" 3 15  0  2 22", " 9 18 13 17  5", "19  8  7 25 23", "20 11 10 24  4", "14 21 16 12  6"].iter().map(|s| s.to_string()).collect(),
///   vec!["14 21 17 24  4", "10 16 15  9 19", "18  8 23 26 20", "22 11 13  6  5", " 2  0 12  3  7"].iter().map(|s| s.to_string()).collect(),
/// ];
/// let outs = vec![
///     vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19],
///     vec![3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6],
///     vec![14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,],
/// ];
/// assert_eq!(day04::unwrap_boards(ins), outs)
/// ```
pub fn unwrap_boards(board_input: Vec<Vec<String>>) -> Vec<Vec<i32>> {
    // step one, combine each vector of strings into a single single
    let combined: Vec<String> = board_input.iter().map(|v| v.join(" ")).collect();

    // step two, split by spaces into numbers
    let numbers: Vec<Vec<i32>> = combined
        .iter()
        .map(|board| {
            board
                .split(" ")
                .map(|b| b.trim())
                .filter(|b| !b.is_empty())
                .map(|b| b.parse().unwrap())
                .collect()
        })
        .collect();

    return numbers;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_board(called: HashSet<u32>) -> BingoBoard {
        let board = BingoBoard {
            board: boardlib::Board::new(
                5,
                5,
                vec![
                    14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2,
                    0, 12, 3, 7,
                ],
            ),
            called_numbers: called,
        };

        return board;
    }

    #[test]
    fn test_has_bingo_and_mark_number() {
        // combining these two, as they naturally work together for an off -> on test.
        let mut board = build_board(HashSet::from([7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21]));
        assert_eq!(board.has_bingo(), false);
        board.mark_number(24);
        assert_eq!(board.has_bingo(), true);
    }

    #[test]
    fn test_score() {
        // combining these two, as they naturally work together for an off -> on test.
        let board = build_board(HashSet::from([7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24]));
        assert_eq!(board.score(), 188);
    }
}
