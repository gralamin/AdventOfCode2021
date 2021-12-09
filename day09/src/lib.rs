extern crate filelib;

pub use filelib::load;

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
                if cur_num < this_num {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_board_number() {
        let board_nums = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let board = Board {
            width: 10,
            height: 5,
            board_numbers: board_nums,
        };
        assert_eq!(board.get_number(0, 0), Some(2));
        assert_eq!(board.get_number(9, 0), Some(0));
        assert_eq!(board.get_number(0, 4), Some(9));
        assert_eq!(board.get_number(9, 4), Some(8));
        assert_eq!(board.get_number(4, 2), Some(7));
        assert_eq!(board.get_number(5, 2), Some(8));
    }

    #[test]
    fn test_get_adjacent_coordinates() {
        let board_nums = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let board = Board {
            width: 10,
            height: 5,
            board_numbers: board_nums,
        };

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
        let board_nums = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let board = Board {
            width: 10,
            height: 5,
            board_numbers: board_nums,
        };
        assert_eq!(check_if_low_point(&board, 0,0), false);
        assert_eq!(check_if_low_point(&board, 1,0), true);
        assert_eq!(check_if_low_point(&board, 2,2), true);
        assert_eq!(check_if_low_point(&board, 0,1), false);
        assert_eq!(check_if_low_point(&board, 0,2), false);
        assert_eq!(check_if_low_point(&board, 4,1), false);
    }
}
