extern crate filelib;
use std::collections::HashMap;

pub use filelib::load_no_blanks;

pub fn parse_line_to_coords(line: &str) -> (i32, i32, i32, i32) {
    let vec_version: Vec<Vec<i32>> = line
        .split("->")
        .map(|pair| {
            pair.split(",")
                .map(|p| p.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let vec_flat: Vec<i32> = vec_version.into_iter().flatten().collect();
    return (vec_flat[0], vec_flat[1], vec_flat[2], vec_flat[3]);
}

fn get_points_on_line(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    return get_points_on_line_full(x1, y1, x2, y2, false);
}

/// Function to get all points a line touches
///
/// This exploits the fact that we are either vertical 1 by 1, or horizontal 1 by 1
/// or diagonally 1 by 1.
fn get_points_on_line_full(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    include_diags: bool,
) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = Vec::new();
    if x1 != x2 && y1 != y2 && !include_diags {
        return points;
    }

    // if we are going to the right, xsign is +1. If no horizontal, 0
    let xsign = (x2 - x1).signum();
    // if we aer going down, ysign is +1. If no vert, 0.
    let ysign = (y2 - y1).signum();
    let range = if xsign != 0 {
        (x1 - x2).abs()
    } else {
        (y1 - y2).abs()
    };
    for i in 0..=range {
        points.push(((x1 + xsign * i), (y1 + ysign * i)));
    }

    return points;
}

trait HydroMapMarkable {
    fn mark_line(&mut self, line_points: Vec<(i32, i32)>);
    fn get_points_with_gte(&self, minimum: i32) -> Vec<(i32, i32)>;
}

struct SparseHydroMap {
    data: HashMap<(i32, i32), i32>,
}

impl HydroMapMarkable for SparseHydroMap {
    fn mark_line(&mut self, line_points: Vec<(i32, i32)>) {
        for pos in line_points {
            if self.data.contains_key(&pos) {
                self.data.insert(pos, self.data[&pos] + 1);
            } else {
                self.data.insert(pos, 1);
            }
        }
    }

    fn get_points_with_gte(&self, minimum: i32) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = Vec::new();
        for (pos, value) in self.data.iter() {
            if value >= &minimum {
                result.push(*pos);
            }
        }
        return result;
    }
}

/// Solution to the first puzzle.
///
/// Mark all the lines in a sparse map, then grab every point in the map with a minimum of two
/// If we needed faster then this, I would have to break out the geometry.
/// ```
/// let inputs = vec![
///                   (0, 9, 5, 9),
///                   (8, 0, 0, 8),
///                   (9, 4, 3, 4),
///                   (2, 2, 2, 1),
///                   (7, 0, 7, 4),
///                   (6, 4, 2, 0),
///                   (0, 9, 2, 9),
///                   (3, 4, 1, 4),
///                   (0, 0, 8, 8),
///                   (5, 5, 8, 2),
/// ];
/// assert_eq!(day05::puzzle_a(&inputs), 5);
/// ```
pub fn puzzle_a(line_pairs: &Vec<(i32, i32, i32, i32)>) -> i32 {
    let mut map = SparseHydroMap {
        data: HashMap::new(),
    };
    for pos_pair in line_pairs {
        let x1 = pos_pair.0;
        let y1 = pos_pair.1;
        let x2 = pos_pair.2;
        let y2 = pos_pair.3;
        let all_points = get_points_on_line(x1, y1, x2, y2);
        map.mark_line(all_points);
    }
    return map.get_points_with_gte(2).len().try_into().unwrap();
}

/// Solution to the second puzzle.
///
/// Same as first, but takes into account 45 degree diagonals
/// ```
/// let inputs = vec![
///                   (0, 9, 5, 9),
///                   (8, 0, 0, 8),
///                   (9, 4, 3, 4),
///                   (2, 2, 2, 1),
///                   (7, 0, 7, 4),
///                   (6, 4, 2, 0),
///                   (0, 9, 2, 9),
///                   (3, 4, 1, 4),
///                   (0, 0, 8, 8),
///                   (5, 5, 8, 2),
/// ];
/// assert_eq!(day05::puzzle_b(&inputs), 12);
/// ```
pub fn puzzle_b(line_pairs: &Vec<(i32, i32, i32, i32)>) -> i32 {
    let mut map = SparseHydroMap {
        data: HashMap::new(),
    };
    for pos_pair in line_pairs {
        let x1 = pos_pair.0;
        let y1 = pos_pair.1;
        let x2 = pos_pair.2;
        let y2 = pos_pair.3;
        let all_points = get_points_on_line_full(x1, y1, x2, y2, true);
        map.mark_line(all_points);
    }
    return map.get_points_with_gte(2).len().try_into().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_points_on_line() {
        let expected_horizontal = vec![(9, 7), (8, 7), (7, 7)];
        let expected_vertical = vec![(1, 1), (1, 2), (1, 3)];
        assert_eq!(get_points_on_line(1, 1, 1, 3), expected_vertical);
        assert_eq!(get_points_on_line(9, 7, 7, 7), expected_horizontal);
    }

    #[test]
    fn test_get_points_on_line_diagnonal() {
        let expected_one = vec![(1, 1), (2, 2), (3, 3)];
        let expected_two = vec![(9, 7), (8, 8), (7, 9)];
        let expected_three = vec![(3, 3), (2, 2), (1, 1)];
        let expected_four = vec![(7, 9), (8, 8), (9, 7)];
        // top left to bottom right
        assert_eq!(get_points_on_line_full(1, 1, 3, 3, true), expected_one);
        // bottom left to top right
        assert_eq!(get_points_on_line_full(9, 7, 7, 9, true), expected_two);
        // bottom right to top left
        assert_eq!(get_points_on_line_full(3, 3, 1, 1, true), expected_three);
        // top right to bottom left
        assert_eq!(get_points_on_line_full(7, 9, 9, 7, true), expected_four);
    }

    #[test]
    fn test_sparse_hydro_map() {
        let mut map = SparseHydroMap {
            data: HashMap::new(),
        };
        let lines = vec![
            get_points_on_line(0, 9, 5, 9),
            get_points_on_line(8, 0, 0, 8),
            get_points_on_line(9, 4, 3, 4),
            get_points_on_line(2, 2, 2, 1),
            get_points_on_line(7, 0, 7, 4),
            get_points_on_line(6, 4, 2, 0),
            get_points_on_line(0, 9, 2, 9),
            get_points_on_line(3, 4, 1, 4),
            get_points_on_line(0, 0, 8, 8),
            get_points_on_line(5, 5, 8, 2),
        ];
        for line in lines {
            map.mark_line(line);
        }
        // Result should be:
        // 0,9   1,9,   2,9  3,4  8,4
        assert_eq!(map.get_points_with_gte(2).len(), 5);
    }

    #[test]
    fn test_sparse_hydro_map_with_diags() {
        let mut map = SparseHydroMap {
            data: HashMap::new(),
        };
        let lines = vec![
            get_points_on_line_full(0, 9, 5, 9, true),
            get_points_on_line_full(8, 0, 0, 8, true),
            get_points_on_line_full(9, 4, 3, 4, true),
            get_points_on_line_full(2, 2, 2, 1, true),
            get_points_on_line_full(7, 0, 7, 4, true),
            get_points_on_line_full(6, 4, 2, 0, true),
            get_points_on_line_full(0, 9, 2, 9, true),
            get_points_on_line_full(3, 4, 1, 4, true),
            get_points_on_line_full(0, 0, 8, 8, true),
            get_points_on_line_full(5, 5, 8, 2, true),
        ];
        for line in lines {
            map.mark_line(line);
        }
        assert_eq!(map.get_points_with_gte(2).len(), 12);
    }

    #[test]
    fn test_parse_line_to_coords() {
        assert_eq!(parse_line_to_coords("6,4 -> 2,0"), (6, 4, 2, 0));
    }
}
