extern crate filelib;

pub use filelib::load;
use std::collections::HashSet;

/// Get size of the board
///
/// ```
/// let input = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
/// assert_eq!(day09::get_board_size(input), (10, 5));
/// ```
pub fn get_board_size(input: &str) -> (i32, i32) {
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
pub fn extract_all_nums(input: &str) -> Vec<i32> {
    return input
        .chars()
        .filter(|x| x.is_numeric())
        .map(|x| x as i32 - '0' as i32)
        .collect();
}

#[derive(Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

trait BoardTraversable {
    fn get_number(&self, x: i32, y: i32) -> Option<i32>;
    fn get_coordinate_by_direction(
        &self,
        x: i32,
        y: i32,
        direction: Direction,
    ) -> (Option<i32>, Option<i32>);
    fn get_adjacent_coordinates(&self, x: i32, y: i32) -> Vec<(i32, i32)>;
}

#[derive(Debug)]
struct Board {
    /* Variable sized boards.
     *
     * width * height = board_numbers.len()
     * index by: x + (y * width)
     * essentially top left corner is 0,0, right and down increases.
     */
    width: i32,
    height: i32,
    board_numbers: Vec<i32>,
}

impl BoardTraversable for Board {
    fn get_number(&self, x: i32, y: i32) -> Option<i32> {
        if y >= self.height || x >= self.width {
            // y cannot exceed height, x cannot exceed width
            return None;
        }
        let pos: usize = (x + (y * self.width)).try_into().unwrap();
        return Some(*(self.board_numbers.iter().nth(pos)?));
    }

    fn get_coordinate_by_direction(
        &self,
        x: i32,
        y: i32,
        direction: Direction,
    ) -> (Option<i32>, Option<i32>) {
        match direction {
            Direction::NORTH => {
                // Can't go North at top edge
                if y == 0 {
                    return (None, None);
                }
                return (Some(x), Some(y - 1));
            }
            Direction::EAST => {
                // Can't go east because at right edge
                if x == self.width - 1 {
                    return (None, None);
                }
                return (Some(x + 1), Some(y));
            }
            Direction::SOUTH => {
                // Can't go down because at bottom
                if y == self.height - 1 {
                    return (None, None);
                }
                return (Some(x), Some(y + 1));
            }
            Direction::WEST => {
                // Can't go east because at left edge
                if x == 0 {
                    return (None, None);
                }
                return (Some(x - 1), Some(y));
            }
        }
    }

    fn get_adjacent_coordinates(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        let (opt_north_x, opt_north_y) = self.get_coordinate_by_direction(x, y, Direction::NORTH);
        let (opt_east_x, opt_east_y) = self.get_coordinate_by_direction(x, y, Direction::EAST);
        let (opt_south_x, opt_south_y) = self.get_coordinate_by_direction(x, y, Direction::SOUTH);
        let (opt_west_x, opt_west_y) = self.get_coordinate_by_direction(x, y, Direction::WEST);
        let mut result: Vec<(i32, i32)> = Vec::new();

        if let Some(north_x) = opt_north_x {
            if let Some(north_y) = opt_north_y {
                result.push((north_x, north_y));
            }
        }

        if let Some(east_x) = opt_east_x {
            if let Some(east_y) = opt_east_y {
                result.push((east_x, east_y));
            }
        }

        if let Some(south_x) = opt_south_x {
            if let Some(south_y) = opt_south_y {
                result.push((south_x, south_y));
            }
        }

        if let Some(west_x) = opt_west_x {
            if let Some(west_y) = opt_west_y {
                result.push((west_x, west_y));
            }
        }

        return result;
    }
}

fn get_risk_level(height: i32) -> i32 {
    return height + 1;
}

fn check_if_low_point(board: &Board, x: i32, y: i32) -> bool {
    if let Some(this_num) = board.get_number(x, y) {
        for (o_x, o_y) in board.get_adjacent_coordinates(x, y) {
            if let Some(cur_num) = board.get_number(o_x, o_y) {
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
pub fn puzzle_a(numbers: Vec<i32>, width: i32, height: i32) -> i32 {
    let board = Board {
        width: width,
        height: height,
        board_numbers: numbers,
    };
    let mut total_risk_level = 0;
    for x in 0..width {
        for y in 0..height {
            let is_lowpoint: bool = check_if_low_point(&board, x, y);
            if is_lowpoint {
                if let Some(n) = board.get_number(x, y) {
                    total_risk_level += get_risk_level(n);
                }
            }
        }
    }
    return total_risk_level;
}

fn find_basin_by_low_point(board: &Board, x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut queue_to_visit: Vec<(i32, i32)> = vec![(x, y)];
    let mut coords: Vec<(i32, i32)> = Vec::new();

    // Search until you hit a 9, Breadth first.
    while !queue_to_visit.is_empty() {
        if let Some(coord_tuple) = queue_to_visit.pop() {
            let (cur_x, cur_y) = coord_tuple;
            if visited.contains(&(cur_x, cur_y)) {
                continue;
            }
            visited.insert((cur_x, cur_y));

            if let Some(value) = board.get_number(cur_x, cur_y) {
                if value == 9 {
                    continue;
                }
            }
            coords.push((cur_x, cur_y));

            let adjacents = board.get_adjacent_coordinates(cur_x, cur_y);
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
pub fn puzzle_b(numbers: Vec<i32>, width: i32, height: i32) -> i32 {
    let board = Board {
        width: width,
        height: height,
        board_numbers: numbers,
    };

    let mut basins: Vec<Vec<(i32, i32)>> = Vec::new();

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

    let len_one: i32 = basins.iter().nth(0).unwrap().len().try_into().unwrap();
    let len_two: i32 = basins.iter().nth(1).unwrap().len().try_into().unwrap();
    let len_three: i32 = basins.iter().nth(2).unwrap().len().try_into().unwrap();
    return len_one * len_two * len_three;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn produce_board() -> Board {
        let board_nums = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let board = Board {
            width: 10,
            height: 5,
            board_numbers: board_nums,
        };
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
            vec![(0, 0), (0, 1), (1, 0)]
        );
        let basin2: Vec<(i32, i32)> = vec![
            (9, 0),
            (8, 0),
            (7, 0),
            (6, 0),
            (5, 0),
            (6, 1),
            (8, 1),
            (9, 1),
            (9, 2),
        ];
        assert_eq!(find_basin_by_low_point(&board, 9, 0), basin2);
        // too complicated to copy lol
        assert_eq!(find_basin_by_low_point(&board, 2, 2).len(), 14);
        let basin4: Vec<(i32, i32)> = vec![
            (6, 4),
            (5, 4),
            (7, 4),
            (8, 4),
            (9, 4),
            (8, 3),
            (7, 3),
            (6, 3),
            (7, 2),
        ];
        assert_eq!(find_basin_by_low_point(&board, 6, 4), basin4);
    }
}
