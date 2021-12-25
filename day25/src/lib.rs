extern crate boardlib;
extern crate filelib;

use crate::boardlib::BoardCoordinate;
use crate::boardlib::BoardTraversable;
use crate::boardlib::Direction;
pub use filelib::load;
use rustc_hash::FxHashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SeaCuc {
    East,
    South,
    Empty,
}

impl std::str::FromStr for SeaCuc {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            ">" => Ok(SeaCuc::East),
            "v" => Ok(SeaCuc::South),
            "." => Ok(SeaCuc::Empty),
            _ => Err(()),
        };
    }
}

/// Parse all sea cucumbers
///
/// ```
/// let s = "v...>>.vv>\n.vv>>.vv..\n>>.>v>...v\n>>v>>.>.v.\nv>v.vv.v..\n>.>>..v...\n.vv..>.>v.\nv.v..>>v.v\n....v..v.>";
/// let result = day25::parse_sea_cucs(&s);
/// assert_eq!(result.len(), 9);
/// assert_eq!(result[0].len(), 10);
/// ```
pub fn parse_sea_cucs(s: &str) -> Vec<Vec<SeaCuc>> {
    let mut result = Vec::new();
    for l in s.lines() {
        let mut current = Vec::new();
        for c in l.chars() {
            if c == ' ' {
                continue;
            }
            current.push(c.to_string().parse().unwrap());
        }
        if current.len() > 0 {
            result.push(current);
        }
    }
    return result;
}

#[derive(Debug)]
struct CucumberBoard {
    board: boardlib::Board<SeaCuc>,
}

impl Clone for CucumberBoard {
    fn clone(&self) -> Self {
        let data = self.board.data_copy();
        return Self {
            board: boardlib::Board::<SeaCuc>::new(
                self.board.get_width(),
                self.board.get_height(),
                data,
            ),
        };
    }
}

impl PartialEq for CucumberBoard {
    fn eq(&self, other: &Self) -> bool {
        return self.board.data_copy() == other.board.data_copy();
    }
}

impl CucumberBoard {
    fn new(data: &Vec<Vec<SeaCuc>>) -> Self {
        let board_data: Vec<SeaCuc> = data.iter().flatten().map(|x| x.clone()).collect();
        return Self {
            board: boardlib::Board::<SeaCuc>::new(data[0].len(), data.len(), board_data),
        };
    }

    fn get(&self, x: usize, y: usize) -> Option<SeaCuc> {
        return self.board.get_value(BoardCoordinate::new(x, y));
    }

    fn step(&mut self) {
        // First handle all the east movement
        self._take_step_loop(&Direction::EAST, &SeaCuc::East);
        self._take_step_loop(&Direction::SOUTH, &SeaCuc::South);
    }

    fn _take_step_loop(&mut self, board_dir: &Direction, value: &SeaCuc) {
        let mut handled: FxHashSet<BoardCoordinate> = FxHashSet::default();
        for coord in self.board.coord_iter() {
            if handled.contains(&coord) {
                continue;
            }
            if let Some(cur) = self.board.get_value(coord) {
                if cur == value.clone() {
                    if let Some(next_coord) =
                        self.board.get_coordinate_by_direction(coord, *board_dir)
                    {
                        if let Some(next) = self.board.get_value(next_coord) {
                            if next == SeaCuc::Empty {
                                self.board.set_value(coord, SeaCuc::Empty);
                                self.board.set_value(next_coord, cur);
                                handled.insert(next_coord);
                                handled.insert(coord);
                            }
                        }
                    } else {
                        // Wrap around logic:
                        let wrap_coord = match board_dir {
                            Direction::EAST => BoardCoordinate::new(0, coord.y),
                            Direction::SOUTH => BoardCoordinate::new(coord.x, 0),
                            Direction::NORTH => {
                                BoardCoordinate::new(coord.x, self.board.get_height() - 1)
                            }
                            Direction::WEST => {
                                BoardCoordinate::new(self.board.get_width() - 1, coord.y)
                            }
                            _ => coord.clone(), // Not bothering to implement Diagonals
                        };
                        // when wrapping, something could be there before we move, handle that case
                        if handled.contains(&wrap_coord) {
                            continue;
                        }
                        if let Some(next) = self.board.get_value(wrap_coord) {
                            if next == SeaCuc::Empty {
                                self.board.set_value(coord, SeaCuc::Empty);
                                self.board.set_value(wrap_coord, cur);
                                handled.insert(wrap_coord);
                                handled.insert(coord);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Calculate when the sea cucumbers stop moving.
///
/// ```
/// let s = "v...>>.vv>\n.vv>>.vv..\n>>.>v>...v\n>>v>>.>.v.\nv>v.vv.v..\n>.>>..v...\n.vv..>.>v.\nv.v..>>v.v\n....v..v.>";
/// let cucs = day25::parse_sea_cucs(&s);
/// assert_eq!(day25::puzzle_a(&cucs), 58);
/// ```
pub fn puzzle_a(cucumbers: &Vec<Vec<SeaCuc>>) -> u32 {
    let mut cur_board = CucumberBoard::new(cucumbers);

    let mut last_board: CucumberBoard;
    let mut step = 0;
    loop {
        step += 1;
        last_board = cur_board.clone();
        cur_board.step();
        if last_board == cur_board {
            return step;
        }
        if step == u32::MAX {
            panic!("Ran out of iterations");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_step() {
        let s = "v..>>>>>...";
        //       01234567890
        let parsed = parse_sea_cucs(&s);
        let mut board = CucumberBoard::new(&parsed);

        board.step();
        assert_eq!(board.get(0, 0), Some(SeaCuc::South));
        assert_eq!(board.get(3, 0), Some(SeaCuc::East));
        assert_eq!(board.get(7, 0), Some(SeaCuc::Empty));
        assert_eq!(board.get(8, 0), Some(SeaCuc::East));

        board.step();
        assert_eq!(board.get(0, 0), Some(SeaCuc::South));
        assert_eq!(board.get(3, 0), Some(SeaCuc::East));
        assert_eq!(board.get(6, 0), Some(SeaCuc::Empty));
        assert_eq!(board.get(7, 0), Some(SeaCuc::East));
        assert_eq!(board.get(8, 0), Some(SeaCuc::Empty));
        assert_eq!(board.get(9, 0), Some(SeaCuc::East));
    }

    #[test]
    fn test_complex_step() {
        let s = "..........\n.>v....v..\n.......>..\n..........\n";
        //       0123456789  0123456789  0123456789  0123456789
        let parsed = parse_sea_cucs(&s);
        let mut board = CucumberBoard::new(&parsed);

        board.step();
        /* Result looks like:
          .......... 0
          .>........ 1
          ..v....v>. 2
          .......... 3
          0123456789 4
        */
        assert_eq!(board.get(1, 1), Some(SeaCuc::East));
        assert_eq!(board.get(2, 1), Some(SeaCuc::Empty));
        assert_eq!(board.get(7, 1), Some(SeaCuc::Empty));
        assert_eq!(board.get(2, 2), Some(SeaCuc::South));
        assert_eq!(board.get(7, 2), Some(SeaCuc::South));
        assert_eq!(board.get(8, 2), Some(SeaCuc::East));
    }

    #[test]
    fn test_wrapping_steps() {
        let s = "..v.\n>..>\n..v.";
        //       0123  0123  0123
        let parsed = parse_sea_cucs(&s);
        let mut board = CucumberBoard::new(&parsed);

        board.step();
        /* Result looks like:
          .... 0
          .>v> 1
          ..v. 2
          0123
        */
        assert_eq!(board.get(2, 0), Some(SeaCuc::Empty));
        assert_eq!(board.get(0, 1), Some(SeaCuc::Empty));
        assert_eq!(board.get(1, 1), Some(SeaCuc::East));
        assert_eq!(board.get(2, 1), Some(SeaCuc::South));
        assert_eq!(board.get(3, 1), Some(SeaCuc::East));
        assert_eq!(board.get(2, 2), Some(SeaCuc::South));

        board.step();
        /* Result looks like:
          ..v. 0
          >>v. 1
          .... 2
          0123
        */
        assert_eq!(board.get(2, 0), Some(SeaCuc::South));
        assert_eq!(board.get(0, 1), Some(SeaCuc::East));
        assert_eq!(board.get(1, 1), Some(SeaCuc::East));
        assert_eq!(board.get(2, 1), Some(SeaCuc::South));
        assert_eq!(board.get(3, 1), Some(SeaCuc::Empty));
        assert_eq!(board.get(2, 2), Some(SeaCuc::Empty));
    }
}
