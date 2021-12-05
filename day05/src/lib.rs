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
fn get_points_on_line_full(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    include_diags: bool,
) -> Vec<(i32, i32)> {
    let mut points: Vec<(i32, i32)> = Vec::new();
    let lower_bound: i32;
    let higher_bound: i32;
    if x1 != x2 && y1 == y2 {
        // horizontal case
        if x1 > x2 {
            lower_bound = x2;
            higher_bound = x1;
        } else {
            lower_bound = x1;
            higher_bound = x2;
        }
        for v in lower_bound..=higher_bound {
            points.push((v, y1));
        }
    } else if x1 == x2 && y1 != y2 {
        // vertical case
        if y1 > y2 {
            lower_bound = y2;
            higher_bound = y1;
        } else {
            lower_bound = y1;
            higher_bound = y2;
        }

        for v in lower_bound..=higher_bound {
            points.push((x1, v));
        }
    } else if include_diags {
        // 45 degree diagonal case
        // 4 possible directions: Bottom left to top right
        if x1 < x2 && y1 > y2 {
            // As x1 increases by one, y1 decrases by one
            for v in x1..=x2 {
                let y = y1 - (v - x1);
                points.push((v, y));
            }
            // direction 2: top left, to bottom right
        } else if x1 < x2 && y1 < y2 {
            // As x1 increases by one, y1 increase by one
            for v in x1..=x2 {
                let y = y1 + (v - x1);
                points.push((v, y));
            }
            // direction 3: Top right to bottom left
        } else if x1 > x2 && y1 < y2 {
            // as x2 increases by one, y2 decreases by one
            for v in x2..=x1 {
                let y = y2 - (v - x2);
                points.push((v, y));
            }
            // Final direction, bottom right to top left
        } else {
            // as x2 increases by one, y2 increases by one
            for v in x2..=x1 {
                let y = y2 + (v - x2);
                points.push((v, y));
            }
        }
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
        let expected_horizontal = vec![(7, 7), (8, 7), (9, 7)];
        let expected_vertical = vec![(1, 1), (1, 2), (1, 3)];
        assert_eq!(get_points_on_line(1, 1, 1, 3), expected_vertical);
        assert_eq!(get_points_on_line(9, 7, 7, 7), expected_horizontal);
    }

    #[test]
    fn test_get_points_on_line_diagnonal() {
        let expected_one = vec![(1, 1), (2, 2), (3, 3)];
        let expected_two = vec![(7, 9), (8, 8), (9, 7)];
        // top left to bottom right
        assert_eq!(get_points_on_line_full(1, 1, 3, 3, true), expected_one);
        // bottom left to top right
        assert_eq!(get_points_on_line_full(9, 7, 7, 9, true), expected_two);
        // bottom right to top left
        assert_eq!(get_points_on_line_full(3, 3, 1, 1, true), expected_one);
        // top right to bottom left
        assert_eq!(get_points_on_line_full(7, 9, 9, 7, true), expected_two);
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
