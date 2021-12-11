extern crate boardlib;
extern crate filelib;
use crate::boardlib::BoardTraversable;
use std::collections::HashSet;

pub use filelib::load;

#[derive(Debug)]
struct OctopusBoard {
    board: boardlib::Board<u32>,
}

impl OctopusBoard {
    fn new(initial_state: Vec<u32>) -> OctopusBoard {
        let height = 10;
        let width = 10;
        let board: boardlib::Board<u32> = boardlib::Board::new(width, height, initial_state);
        return OctopusBoard { board: board };
    }

    fn take_a_step(&mut self) -> u32 {
        let mut have_flashed: HashSet<boardlib::BoardCoordinate> = HashSet::new();
        // Step 1, increase energy level of each by one.
        for current_coord in self.board.coord_iter() {
            self.increment_location(current_coord);
        }
        // Step 2, iterate until we see no flashes
        let mut flashes_added = true;
        while flashes_added {
            flashes_added = false;
            for current_coord in self.board.coord_iter() {
                if have_flashed.contains(&current_coord) {
                    continue;
                }
                if let Some(v) = self.board.get_value(current_coord) {
                    if v <= 9 {
                        continue;
                    }
                    flashes_added = true;
                    have_flashed.insert(current_coord);
                    for adjacent in self.board.get_adjacent_coordinates(current_coord) {
                        self.increment_location(adjacent);
                    }
                    for adjacent in self.board.get_diag_adjacent_coordinates(current_coord) {
                        self.increment_location(adjacent);
                    }
                }
            }
        }

        // Step 3, set all flashes to 0
        let flashes: u32 = have_flashed.len().try_into().unwrap();
        for current_coord in have_flashed {
            self.board.set_value(current_coord, 0);
        }
        return flashes;
    }

    fn increment_location(&mut self, pos: boardlib::BoardCoordinate) {
        if let Some(v) = self.board.get_value(pos) {
            self.board.set_value(pos, v + 1);
        }
    }
}

/// Get all num chars from an input string as a unique integer
/// ```
/// let expected = vec![
///     2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
///     2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
/// ];
/// let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
/// assert_eq!(day11::extract_all_nums(input), expected);
/// ```
pub fn extract_all_nums(input: &str) -> Vec<u32> {
    return input
        .chars()
        .filter(|x| x.is_numeric())
        .map(|x| x as u32 - '0' as u32)
        .collect();
}

/// Count the number of flashes that occur in 100 moves
///
/// ```
/// let v = vec![
///     5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
///     3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6,
///     4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8,
///     5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
/// ];
/// assert_eq!(day11::puzzle_a(&v), 1656);
/// ```
pub fn puzzle_a(values: &Vec<u32>) -> u32 {
    let mut board = OctopusBoard::new(values.to_vec());
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += board.take_a_step();
    }
    return flashes;
}

/// Find the first step on which the octopuses sync (100 flashes)
///
/// ```
/// let v = vec![
///     5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
///     3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6,
///     4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8,
///     5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
/// ];
/// assert_eq!(day11::puzzle_b(&v), 195);
/// ```
pub fn puzzle_b(values: &Vec<u32>) -> u32 {
    let mut board = OctopusBoard::new(values.to_vec());
    let mut flashes: u32 = 0;
    let mut step: u32 = 0;
    while flashes != 100 {
        step += 1;
        flashes = board.take_a_step();
    }
    return step;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_board() -> OctopusBoard {
        let v = vec![
            5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1, 7,
            3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2, 4, 6,
            4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6, 8, 4, 8,
            5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
        ];
        return OctopusBoard::new(v);
    }

    #[test]
    fn test_take_a_step() {
        let mut flashes = 0;
        let mut board = make_board();
        flashes += board.take_a_step();
        assert_eq!(flashes, 0);
        flashes += board.take_a_step();
        assert_eq!(flashes, 35);
    }
}
