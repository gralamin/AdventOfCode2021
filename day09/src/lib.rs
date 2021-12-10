extern crate boardlib;
extern crate filelib;

use crate::boardlib::BoardCoordinate;
use crate::boardlib::BoardTraversable;
pub use filelib::load;
use std::collections::HashSet;

/// Get size of the board
///
/// ```
/// let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
/// assert_eq!(day09::get_board_size(input), (10, 5));
/// ```
pub fn get_board_size(input: &str) -> (usize, usize) {
    let nonempty_lines: Vec<&str> = input.lines().filter(|x| !x.trim().is_empty()).collect();
    let height = nonempty_lines.len();
    let width = nonempty_lines.iter().nth(0).unwrap().trim().len();

    return (width.try_into().unwrap(), height.try_into().unwrap());
}

/// Get all num chars from an input string as a unique integer
/// ```
/// let expected = vec![
///     2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
///     2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
/// ];
/// let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
/// assert_eq!(day09::extract_all_nums(input), expected);
/// ```
pub fn extract_all_nums(input: &str) -> Vec<usize> {
    return input
        .chars()
        .filter(|x| x.is_numeric())
        .map(|x| x as usize - '0' as usize)
        .collect();
}

fn get_risk_level(height: usize) -> usize {
    return height + 1;
}

fn check_if_low_point(board: &boardlib::Board<usize>, x: usize, y: usize) -> bool {
    let pos = BoardCoordinate::new(x, y);
    if let Some(this_num) = board.get_value(pos) {
        for adjacent_coordinate in board.get_adjacent_coordinates(pos) {
            if let Some(cur_num) = board.get_value(adjacent_coordinate) {
                if cur_num <= this_num {
                    return false;
                }
            }
        }
    } else {
        return false;
    }
    return true;
}

/// Find each lowpoint (no adjacent smaller value), calculate the risk level, and sum them
///
/// ```
/// let board_nums = vec![
///     2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
///     2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
/// ];
/// assert_eq!(day09::puzzle_a(board_nums, 10, 5), 15);
/// ```
pub fn puzzle_a(numbers: Vec<usize>, width: usize, height: usize) -> usize {
    let board: boardlib::Board<usize> = boardlib::Board::new(width, height, numbers);
    let mut total_risk_level = 0;
    for x in 0..width {
        for y in 0..height {
            let is_lowpoint: bool = check_if_low_point(&board, x, y);
            if is_lowpoint {
                if let Some(n) = board.get_value(BoardCoordinate::new(x, y)) {
                    total_risk_level += get_risk_level(n);
                }
            }
        }
    }
    return total_risk_level;
}

fn find_basin_by_low_point(
    board: &boardlib::Board<usize>,
    x: usize,
    y: usize,
) -> Vec<BoardCoordinate> {
    let mut visited: HashSet<BoardCoordinate> = HashSet::new();
    let mut queue_to_visit: Vec<BoardCoordinate> = vec![BoardCoordinate::new(x, y)];
    let mut coords: Vec<BoardCoordinate> = Vec::new();

    // Search until you hit a 9, Breadth first.
    while !queue_to_visit.is_empty() {
        if let Some(coord_tuple) = queue_to_visit.pop() {
            if visited.contains(&coord_tuple) {
                continue;
            }
            visited.insert(coord_tuple);

            if let Some(value) = board.get_value(coord_tuple) {
                if value == 9 {
                    continue;
                }
            }
            coords.push(coord_tuple);

            let adjacents = board.get_adjacent_coordinates(coord_tuple);
            for coord in adjacents {
                queue_to_visit.push(coord);
            }
        }
    }

    return coords;
}

/// From the low points, find the basins, get the largest three, and multiply together
///
/// ```
/// let board_nums = vec![
///     2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
///     2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
/// ];
/// assert_eq!(day09::puzzle_b(board_nums, 10, 5), 1134);
/// ```
pub fn puzzle_b(numbers: Vec<usize>, width: usize, height: usize) -> usize {
    let board: boardlib::Board<usize> = boardlib::Board::new(width, height, numbers);

    let mut basins: Vec<Vec<BoardCoordinate>> = Vec::new();

    for x in 0..width {
        for y in 0..height {
            let is_lowpoint: bool = check_if_low_point(&board, x, y);
            if is_lowpoint {
                basins.push(find_basin_by_low_point(&board, x, y));
            }
        }
    }

    basins.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    basins.reverse();

    let len_one: usize = basins.iter().nth(0).unwrap().len().try_into().unwrap();
    let len_two: usize = basins.iter().nth(1).unwrap().len().try_into().unwrap();
    let len_three: usize = basins.iter().nth(2).unwrap().len().try_into().unwrap();
    return len_one * len_two * len_three;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn produce_board() -> boardlib::Board<usize> {
        let board_nums = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let board: boardlib::Board<usize> = boardlib::Board::new(10, 5, board_nums);
        return board;
    }

    #[test]
    fn test_get_board_number() {
        let board = produce_board();
        assert_eq!(board.get_number(0, 0), Some(2));
        assert_eq!(board.get_number(9, 0), Some(0));
        assert_eq!(board.get_number(0, 4), Some(9));
        assert_eq!(board.get_number(9, 4), Some(8));
        assert_eq!(board.get_number(4, 2), Some(7));
        assert_eq!(board.get_number(5, 2), Some(8));
    }

    #[test]
    fn test_get_adjacent_coordinates() {
        let board = produce_board();
        assert_eq!(board.get_adjacent_coordinates(0, 0), vec![(1, 0), (0, 1)]);
        assert_eq!(board.get_adjacent_coordinates(9, 0), vec![(9, 1), (8, 0)]);
        assert_eq!(board.get_adjacent_coordinates(0, 4), vec![(0, 3), (1, 4)]);
        assert_eq!(board.get_adjacent_coordinates(9, 4), vec![(9, 3), (8, 4)]);
    }

    #[test]
    fn test_get_risk_level() {
        assert_eq!(get_risk_level(1), 2);
        assert_eq!(get_risk_level(0), 1);
        assert_eq!(get_risk_level(5), 6);
    }

    #[test]
    fn test_check_if_low_point() {
        let board = produce_board();
        assert_eq!(check_if_low_point(&board, 0, 0), false);
        assert_eq!(check_if_low_point(&board, 1, 0), true);
        assert_eq!(check_if_low_point(&board, 2, 2), true);
        assert_eq!(check_if_low_point(&board, 0, 1), false);
        assert_eq!(check_if_low_point(&board, 0, 2), false);
        assert_eq!(check_if_low_point(&board, 4, 1), false);
    }

    #[test]
    fn test_find_basin_by_low_point() {
        let board = produce_board();
        assert_eq!(
            find_basin_by_low_point(&board, 0, 0),
            vec![
                BoardCoordinate::new(0, 0),
                BoardCoordinate::new(0, 1),
                BoardCoordinate::new(1, 0)
            ]
        );
        let basin2: Vec<(usize, usize)> = vec![
            BoardCoordinate::new(9, 0),
            BoardCoordinate::new(8, 0),
            BoardCoordinate::new(7, 0),
            BoardCoordinate::new(6, 0),
            BoardCoordinate::new(5, 0),
            BoardCoordinate::new(6, 1),
            BoardCoordinate::new(8, 1),
            BoardCoordinate::new(9, 1),
            BoardCoordinate::new(9, 2),
        ];
        assert_eq!(find_basin_by_low_point(&board, 9, 0), basin2);
        // too complicated to copy lol
        assert_eq!(find_basin_by_low_point(&board, 2, 2).len(), 14);
        let basin4: Vec<(usize, usize)> = vec![
            BoardCoordinate::new(6, 4),
            BoardCoordinate::new(5, 4),
            BoardCoordinate::new(7, 4),
            BoardCoordinate::new(8, 4),
            BoardCoordinate::new(9, 4),
            BoardCoordinate::new(8, 3),
            BoardCoordinate::new(7, 3),
            BoardCoordinate::new(6, 3),
            BoardCoordinate::new(7, 2),
        ];
        assert_eq!(find_basin_by_low_point(&board, 6, 4), basin4);
    }
}
