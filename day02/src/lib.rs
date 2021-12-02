mod filelib;

use std::str::FromStr;

pub use crate::filelib::load_no_blanks;

#[derive(Debug, PartialEq)]
pub enum SubDirection {
    Forward,
    Down,
    Up,
}

impl FromStr for SubDirection {
    type Err = ();

    fn from_str(input: &str) -> Result<SubDirection, Self::Err> {
        match input {
            "forward" => Ok(SubDirection::Forward),
            "up" => Ok(SubDirection::Up),
            "down" => Ok(SubDirection::Down),
            _ => Err(()),
        }
    }
}

/// Converts lines from the input file to sub directions
///
/// ```
/// let values = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"];
/// let expected = vec![(day02::SubDirection::Forward, 5),
///                     (day02::SubDirection::Down, 5),
///                     (day02::SubDirection::Forward, 8),
///                     (day02::SubDirection::Up, 3),
///                     (day02::SubDirection::Down, 8),
///                     (day02::SubDirection::Forward, 2)];
/// assert_eq!(day02::directional_commands_from_strs(values), expected);
/// ```
pub fn directional_commands_from_strs(str_list: Vec<&str>) -> Vec<(SubDirection, i32)> {
    let mut result: Vec<(SubDirection, i32)> = Vec::new();
    for cur_str in str_list {
        let mut split_iter = cur_str.split(" ");
        let dir = SubDirection::from_str(split_iter.next().unwrap()).unwrap();
        let value = split_iter.next().unwrap().parse::<i32>().unwrap();
        result.push((dir, value));
    }
    return result;
}

trait SubPosition {
    fn forward(&mut self, by: i32);
    fn down(&mut self, by: i32);
    fn up(&mut self, by: i32);
}

#[derive(Debug)]
struct Sub {
    horizontal_pos: i32,
    depth: i32,
}

impl SubPosition for Sub {
    fn forward(&mut self, by: i32) {
        self.horizontal_pos += by;
    }

    fn down(&mut self, by: i32) {
        self.depth += by;
    }

    fn up(&mut self, by: i32) {
        self.depth -= by;
    }
}

/// Move the sub according to the directions.
///
/// Assumes the directions have already been parsed.
///
/// ```
/// let steps = vec![(day02::SubDirection::Forward, 5),
///                  (day02::SubDirection::Down, 5),
///                  (day02::SubDirection::Forward, 8),
///                  (day02::SubDirection::Up, 3),
///                  (day02::SubDirection::Down, 8),
///                  (day02::SubDirection::Forward, 2)];
/// assert_eq!(day02::puzzle_a(steps), 150);
/// ```
pub fn puzzle_a(steps: Vec<(SubDirection, i32)>) -> i32 {
    let mut sub = Sub {
        horizontal_pos: 0,
        depth: 0,
    };
    for command in steps {
        let dir = command.0;
        let by = command.1;
        match dir {
            SubDirection::Forward => sub.forward(by),
            SubDirection::Down => sub.down(by),
            SubDirection::Up => sub.up(by),
        }
    }
    return sub.horizontal_pos * sub.depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_movement() {
        let mut sub = Sub {
            horizontal_pos: 0,
            depth: 0,
        };
        sub.forward(5);
        assert_eq!(sub.horizontal_pos, 5);
        sub.down(5);
        assert_eq!(sub.depth, 5);
        sub.forward(8);
        assert_eq!(sub.horizontal_pos, 13);
        sub.up(3);
        assert_eq!(sub.depth, 2);
        sub.down(8);
        assert_eq!(sub.depth, 10);
        sub.forward(2);
        assert_eq!(sub.horizontal_pos, 15);
    }
}
