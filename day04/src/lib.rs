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
