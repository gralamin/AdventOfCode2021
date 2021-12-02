extern crate filelib;
extern crate submarinelib;

use submarinelib::AimingSub;
use submarinelib::AimlessSub;
use submarinelib::SubDirection;
use submarinelib::SubPosition;

pub use filelib::load_no_blanks;
pub use submarinelib::directional_commands_from_strs;

/// Move the sub according to the directions.
///
/// Assumes the directions have already been parsed.
///
/// ```
/// let steps = vec![(submarinelib::SubDirection::Forward, 5),
///                  (submarinelib::SubDirection::Down, 5),
///                  (submarinelib::SubDirection::Forward, 8),
///                  (submarinelib::SubDirection::Up, 3),
///                  (submarinelib::SubDirection::Down, 8),
///                  (submarinelib::SubDirection::Forward, 2)];
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
/// let steps = vec![(submarinelib::SubDirection::Forward, 5),
///                  (submarinelib::SubDirection::Down, 5),
///                  (submarinelib::SubDirection::Forward, 8),
///                  (submarinelib::SubDirection::Up, 3),
///                  (submarinelib::SubDirection::Down, 8),
///                  (submarinelib::SubDirection::Forward, 2)];
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
