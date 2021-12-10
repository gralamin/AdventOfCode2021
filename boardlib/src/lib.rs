#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardCoordinate {
    x: usize,
    y: usize,
}

impl BoardCoordinate {
    pub fn new(x: usize, y: usize) -> BoardCoordinate {
        return BoardCoordinate { x: x, y: y };
    }
}

impl std::ops::Add for BoardCoordinate {
    type Output = BoardCoordinate;

    fn add(self, other: BoardCoordinate) -> BoardCoordinate {
        return BoardCoordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug)]
pub struct Board<T: Copy> {
    /* Variable sized boards.
     *
     * width * height = board_numbers.len()
     * index by: x + (y * width)
     * essentially top left corner is 0,0, right and down increases.
     */
    width: usize,
    height: usize,
    values: Vec<T>,
}

impl<T: Copy> Board<T> {
    pub fn new(width: usize, height: usize, values: Vec<T>) -> Board<T> {
        return Board {
            width: width,
            height: height,
            values: values,
        };
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }
}

#[derive(Debug)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

pub trait BoardTraversable {
    type Item;

    fn get_value(&self, pos: BoardCoordinate) -> Option<Self::Item>;
    fn get_coordinate_by_direction(
        &self,
        pos: BoardCoordinate,
        direction: Direction,
    ) -> Option<BoardCoordinate>;
    fn get_adjacent_coordinates(&self, pos: BoardCoordinate) -> Vec<BoardCoordinate>;
}

impl<T: Copy> BoardTraversable for Board<T> {
    type Item = T;

    fn get_value(&self, pos: BoardCoordinate) -> Option<Self::Item> {
        if pos.y >= self.height || pos.x >= self.width {
            // y cannot exceed height, x cannot exceed width
            return None;
        }
        let pos: usize = pos.x + pos.y * self.width;
        return Some(*(self.values.iter().nth(pos)?));
    }

    fn get_coordinate_by_direction(
        &self,
        pos: BoardCoordinate,
        direction: Direction,
    ) -> Option<BoardCoordinate> {
        let mut possible_y: Option<usize> = Some(pos.y);
        let mut possible_x: Option<usize> = Some(pos.x);
        match direction {
            Direction::NORTH => possible_y = pos.y.checked_sub(1),
            Direction::EAST => possible_x = pos.x.checked_add(1),
            Direction::SOUTH => possible_y = pos.y.checked_add(1),
            Direction::WEST => possible_x = pos.x.checked_sub(1),
        }
        if let Some(new_x) = possible_x {
            if let Some(new_y) = possible_y {
                if new_x > self.width - 1 || new_y > self.height - 1 {
                    return None;
                }
                return Some(BoardCoordinate::new(new_x, new_y));
            }
        }
        return None;
    }

    fn get_adjacent_coordinates(&self, pos: BoardCoordinate) -> Vec<BoardCoordinate> {
        let opt_north = self.get_coordinate_by_direction(pos, Direction::NORTH);
        let opt_east = self.get_coordinate_by_direction(pos, Direction::EAST);
        let opt_south = self.get_coordinate_by_direction(pos, Direction::SOUTH);
        let opt_west = self.get_coordinate_by_direction(pos, Direction::WEST);
        let mut result: Vec<BoardCoordinate> = Vec::new();
        let options = vec![opt_north, opt_east, opt_south, opt_west];

        for possible_pos in options {
            if let Some(cur_pos) = possible_pos {
                result.push(cur_pos);
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn produce_board() -> Board<i32> {
        let board_nums = vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ];
        let board: Board<i32> = Board::new(10, 5, board_nums);
        return board;
    }

    #[test]
    fn test_get_board_number() {
        let board = produce_board();
        assert_eq!(board.get_value(BoardCoordinate::new(0, 0)), Some(2));
        assert_eq!(board.get_value(BoardCoordinate::new(9, 0)), Some(0));
        assert_eq!(board.get_value(BoardCoordinate::new(0, 4)), Some(9));
        assert_eq!(board.get_value(BoardCoordinate::new(9, 4)), Some(8));
        assert_eq!(board.get_value(BoardCoordinate::new(4, 2)), Some(7));
        assert_eq!(board.get_value(BoardCoordinate::new(5, 2)), Some(8));
    }

    #[test]
    fn test_get_adjacent_coordinates() {
        let board = produce_board();
        assert_eq!(
            board.get_adjacent_coordinates(BoardCoordinate::new(0, 0)),
            vec![BoardCoordinate::new(1, 0), BoardCoordinate::new(0, 1)]
        );
        assert_eq!(
            board.get_adjacent_coordinates(BoardCoordinate::new(9, 0)),
            vec![BoardCoordinate::new(9, 1), BoardCoordinate::new(8, 0)]
        );
        assert_eq!(
            board.get_adjacent_coordinates(BoardCoordinate::new(0, 4)),
            vec![BoardCoordinate::new(0, 3), BoardCoordinate::new(1, 4)]
        );
        assert_eq!(
            board.get_adjacent_coordinates(BoardCoordinate::new(9, 4)),
            vec![BoardCoordinate::new(9, 3), BoardCoordinate::new(8, 4)]
        );
    }

    #[test]
    fn test_get_width() {
        let board = produce_board();
        assert_eq!(board.get_width(), 10);
    }

    #[test]
    fn test_get_height() {
        let board = produce_board();
        assert_eq!(board.get_height(), 5);
    }

    #[test]
    fn test_add_coords() {
        let a = BoardCoordinate::new(3, 5);
        let b = BoardCoordinate::new(7, 11);
        let expected = BoardCoordinate::new(10, 16);
        assert_eq!(a + b, expected);
    }
}
