mod filelib;
mod sub;

use crate::sub::AimingSub;
use crate::sub::AimlessSub;
use crate::sub::SubDirection;
use crate::sub::SubPosition;

pub use crate::filelib::load_no_blanks;
pub use crate::sub::directional_commands_from_strs;

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
    let mut sub = AimlessSub {
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

/// Move the sub according to the directions for a puzzle b version.
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
/// assert_eq!(day02::puzzle_b(steps), 900);
/// ```
pub fn puzzle_b(steps: Vec<(SubDirection, i32)>) -> i32 {
    let mut sub = AimingSub {
        horizontal_pos: 0,
        depth: 0,
        aim: 0,
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
