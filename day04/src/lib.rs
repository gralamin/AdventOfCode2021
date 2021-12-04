extern crate filelib;
use std::collections::HashSet;

pub use filelib::load;

trait SolvableBingoBoard {
    fn has_bingo(&self) -> bool;
    fn score(&self) -> i32;
    fn mark_number(&mut self, num: i32);
}

trait BoardIndexable {
    fn get_number(&self, x: i32, y: i32) -> i32;
}

#[derive(Debug)]
struct BingoBoard {
    /* Variable sized boards.
     *
     * width * height = board_numbers.len()
     * index by: x + (y * width)
     * essentially top left corner is 0,0, right and down increases.
     */
    width: i32,
    height: i32,
    board_numbers: Vec<i32>,

    /* Game state variables
     *
     * We currently need to know which numbers are marked only.
     */
    called_numbers: HashSet<i32>,
}

impl BoardIndexable for BingoBoard {
    fn get_number(&self, x: i32, y: i32) -> i32 {
        if y >= self.height || x >= self.width {
            // y cannot exceed height, x cannot exceed width
            return -999;
        }
        let pos: usize = (x + (y * self.width)).try_into().unwrap();
        return *(self.board_numbers.iter().nth(pos).unwrap());
    }
}

impl SolvableBingoBoard for BingoBoard {
    /// Have bingo if all rows or columns are marked
    ///
    /// Worst case O(n) (no numbers marked, read entire board twice)
    /// Assuming lookup of numbers in my set_hash is O(1)
    fn has_bingo(&self) -> bool {
        // columns: on a given x go down all the ys
        for x in 0..self.width {
            let mut is_bingo = true;
            for y in 0..self.height {
                let cur_num = self.get_number(x, y);
                if !self.called_numbers.contains(&cur_num) {
                    // find one miss, skip
                    is_bingo = false;
                    break;
                }
            }
            // first bingo we find, quit
            if is_bingo {
                return true;
            }
        }

        // No column has bingo, check rows.
        for y in 0..self.height {
            let mut is_bingo = true;
            for x in 0..self.width {
                let cur_num = self.get_number(x, y);
                if !self.called_numbers.contains(&cur_num) {
                    // find one miss, skip
                    is_bingo = false;
                    break;
                }
            }
            // first bingo we find, quit
            if is_bingo {
                return true;
            }
        }
        return false;
    }

    /// Score is sum of all unmarked numbers
    ///
    /// O(n) by nature of iterating over the board.
    /// Assuming lookup of numbers in my set_hash is O(1)
    fn score(&self) -> i32 {
        let mut total = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let cur_num = self.get_number(x, y);
                if !self.called_numbers.contains(&cur_num) {
                    total += cur_num;
                }
            }
        }
        return total;
    }

    fn mark_number(&mut self, num: i32) {
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
pub fn puzzle_a(called_numbers: &Vec<i32>, boards: &Vec<Vec<i32>>) -> i32 {
    // Theoretically we could skip to the first 4 numbers called, as there cannot be a bingo until the
    // fifth number, but not going to bother optimizing that.
    let mut board_vec: Vec<BingoBoard> = boards
        .iter()
        .map(|nums| BingoBoard {
            height: 5,
            width: 5,
            board_numbers: nums.to_vec(),
            called_numbers: HashSet::new(),
        })
        .collect();

    for calling_number in called_numbers.iter() {
        // iterate by index so we can mutate structure safely
        for i in 0..board_vec.len() {
            board_vec[i].mark_number(*calling_number);
            if board_vec[i].has_bingo() {
                return board_vec[i].score() * (*calling_number);
            }
        }
    }
    // If we don't get a bingo throw a number I know is wrong
    return -999;
}

pub fn split_lines_by_blanks(lines: &str) -> Vec<Vec<String>> {
    let result: Vec<Vec<String>> = Vec::new();

    let mut cur_break: Vec<String> = Vec::new();
    for cur_line in lines.lines() {}

    return result;
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

/// Input parsing, split the first lines into a bunch of numbers
///
/// TODO move this over to filelib
/// ```
/// let ins = vec![vec!["7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string()]];
/// let outs = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];
/// assert_eq!(day04::parse_csv_i32_lines(ins), outs);
/// ```
pub fn parse_csv_i32_lines(lines: Vec<Vec<String>>) -> Vec<i32> {
    // First, flatten a layer
    let flattened_lines: Vec<String> = lines.into_iter().flatten().collect();
    let number_lines: Vec<Vec<i32>> = flattened_lines
        .iter()
        .map(|line| {
            line.split(",")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    let numbers: Vec<i32> = number_lines.into_iter().flatten().collect();
    return numbers;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_number() {
        let board = BingoBoard {
            height: 5,
            width: 5,
            board_numbers: vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
            called_numbers: HashSet::from([7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21]),
        };
        assert_eq!(board.get_number(0, 0), 14);
        assert_eq!(board.get_number(4, 0), 4);
        assert_eq!(board.get_number(0, 4), 2);
        assert_eq!(board.get_number(4, 4), 7);
        assert_eq!(board.get_number(2, 2), 23);
    }

    #[test]
    fn test_has_bingo_and_mark_number() {
        // combining these two, as they naturally work together for an off -> on test.
        let mut board = BingoBoard {
            height: 5,
            width: 5,
            board_numbers: vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
            called_numbers: HashSet::from([7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21]),
        };
        assert_eq!(board.has_bingo(), false);
        board.mark_number(24);
        assert_eq!(board.has_bingo(), true);
    }

    #[test]
    fn test_score() {
        // combining these two, as they naturally work together for an off -> on test.
        let board = BingoBoard {
            height: 5,
            width: 5,
            board_numbers: vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ],
            called_numbers: HashSet::from([7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24]),
        };
        assert_eq!(board.score(), 188);
    }
}
