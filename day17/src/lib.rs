pub use filelib::load;

use std::cmp::max;

/*       +y
 *       |
 *       |
 *       |
 *       +--------------- +x
 *       |
 *       |
 *      -y
 */

/// Parse the target area
///
/// Returns lower left corner and upper right corner.
/// ```
/// let input = "target area: x=20..30, y=-10..-5";
/// assert_eq!(day17::load_target_area(input), (20, -10, 30, -5))
/// ```
pub fn load_target_area(input: &str) -> (i32, i32, i32, i32) {
    let (_, coords_str) = input.split_once(" ").unwrap();
    let (x_str, y_str) = coords_str.split_once(",").unwrap();
    let (_, x_coord_str) = x_str.trim().split_once("=").unwrap();
    let (x1_str, x2_str) = x_coord_str.split_once("..").unwrap();
    let x1: i32 = x1_str.parse().unwrap();
    let x2: i32 = x2_str.parse().unwrap();

    let (_, y_coord_str) = y_str.trim().split_once("=").unwrap();
    let (y1_str, y2_str) = y_coord_str.split_once("..").unwrap();
    let y1: i32 = y1_str.parse().unwrap();
    let y2: i32 = y2_str.parse().unwrap();

    let lesser_x: i32;
    let lesser_y: i32;
    let upper_x: i32;
    let upper_y: i32;
    if x1 < x2 {
        upper_x = x2;
        lesser_x = x1;
    } else {
        upper_x = x1;
        lesser_x = x2;
    }

    if y1 < y2 {
        upper_y = y2;
        lesser_y = y1;
    } else {
        upper_y = y1;
        lesser_y = y2;
    }

    return (lesser_x, lesser_y, upper_x, upper_y);
}

fn simulate(
    dx: i32,
    dy: i32,
    target_x1: i32,
    target_y1: i32,
    target_x2: i32,
    target_y2: i32,
) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut ans = 0;
    let mut cur_dx = dx;
    let mut cur_dy = dy;
    // If we aren't too far right, and not too far down, continue.
    while x < target_x2 && y >= target_y1 {
        x += cur_dx;
        y += cur_dy;
        cur_dy -= 1;
        ans = max(ans, y);
        if cur_dx > 0 {
            cur_dx -= 1;
        } else if cur_dx < 0 {
            cur_dx += 1;
        }
        if target_x1 <= x && x <= target_x2 && target_y1 <= y && y <= target_y2 {
            return ans;
        }
    }
    return -1;
}

/// Find highest y, via bounds testing and simulating
///
/// ```
/// assert_eq!(day17::puzzle_a(20, -10, 30, -5), 45)
/// ```
pub fn puzzle_a(target_x1: i32, target_y1: i32, target_x2: i32, target_y2: i32) -> i32 {
    let mut max_y = 0;
    for y in target_y1..=target_y1.abs() {
        for x in 0..=target_x2 {
            let result = simulate(x, y, target_x1, target_y1, target_x2, target_y2);
            if result != -1 {
                max_y = max(max_y, result);
            }
        }
    }
    return max_y;
}

/// Find number of possible velocities
///
/// ```
/// assert_eq!(day17::puzzle_b(20, -10, 30, -5), 112)
/// ```
pub fn puzzle_b(target_x1: i32, target_y1: i32, target_x2: i32, target_y2: i32) -> i32 {
    let mut num_hit = 0;
    for y in target_y1..=target_y1.abs() {
        for x in 0..=target_x2 {
            let result = simulate(x, y, target_x1, target_y1, target_x2, target_y2);
            if result != -1 {
                num_hit += 1;
            }
        }
    }
    return num_hit;
}
