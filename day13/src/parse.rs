use boardlib::BoardCoordinate;
use rustc_hash::FxHashSet;

use crate::Fold;

/// Parse the folds instructions
///
/// ```
/// let inputs = vec![
///     "fold along y=7".to_string(),
///     "fold along x=5".to_string(),
/// ];
/// let expected = vec![day13::Fold::Horizontal(7), day13::Fold::Vertical(5)];
/// assert_eq!(day13::parse_folds(&inputs), expected);
/// ```
pub fn parse_folds(inputs: &Vec<String>) -> Vec<Fold> {
    let mut folds: Vec<Fold> = Vec::new();
    for line in inputs {
        if let Some(last) = line.trim().split(" ").last() {
            let (along, value_str) = last.split_once("=").unwrap();
            let value = value_str.trim().parse::<usize>().unwrap();
            if along == "x" {
                folds.push(Fold::Vertical(value));
            } else if along == "y" {
                folds.push(Fold::Horizontal(value));
            }
        }
    }
    return folds;
}

/// Parse the coords list
///
/// ```
/// use rustc_hash::FxHashSet;
/// use boardlib::BoardCoordinate;
/// let inputs = vec![
///     "6,10".to_string(),
///     "0,14".to_string(),
///     "9,10".to_string(),
///     "0,3".to_string(),
///     "10,4".to_string(),
///     "4,11".to_string(),
///     "6,0".to_string(),
///     "6,12".to_string(),
///     "4,1".to_string(),
///     "0,13".to_string(),
///     "10,12".to_string(),
///     "3,4".to_string(),
///     "3,0".to_string(),
///     "8,4".to_string(),
///     "1,10".to_string(),
///     "2,14".to_string(),
///     "8,10".to_string(),
///     "9,0".to_string(),
/// ];
/// let mut dot_coords: FxHashSet<BoardCoordinate> = FxHashSet::default();
/// dot_coords.insert(BoardCoordinate::new(6, 10));
/// dot_coords.insert(BoardCoordinate::new(0, 14));
/// dot_coords.insert(BoardCoordinate::new(9, 10));
/// dot_coords.insert(BoardCoordinate::new(0, 3));
/// dot_coords.insert(BoardCoordinate::new(10, 4));
/// dot_coords.insert(BoardCoordinate::new(4, 11));
/// dot_coords.insert(BoardCoordinate::new(6, 0));
/// dot_coords.insert(BoardCoordinate::new(6, 12));
/// dot_coords.insert(BoardCoordinate::new(4, 1));
/// dot_coords.insert(BoardCoordinate::new(0, 13));
/// dot_coords.insert(BoardCoordinate::new(10, 12));
/// dot_coords.insert(BoardCoordinate::new(3, 4));
/// dot_coords.insert(BoardCoordinate::new(3, 0));
/// dot_coords.insert(BoardCoordinate::new(8, 4));
/// dot_coords.insert(BoardCoordinate::new(1, 10));
/// dot_coords.insert(BoardCoordinate::new(2, 14));
/// dot_coords.insert(BoardCoordinate::new(8, 10));
/// dot_coords.insert(BoardCoordinate::new(9, 0));
/// assert_eq!(day13::parse_coords(&inputs), dot_coords);
/// ```
pub fn parse_coords(inputs: &Vec<String>) -> FxHashSet<BoardCoordinate> {
    let mut coords: FxHashSet<BoardCoordinate> = FxHashSet::default();
    for line in inputs {
        let (x_str, y_str) = line.split_once(",").unwrap();
        let x = x_str.trim().parse::<usize>().unwrap();
        let y = y_str.trim().parse::<usize>().unwrap();
        coords.insert(BoardCoordinate::new(x, y));
    }
    return coords;
}

pub fn coords_to_bool(
    dot_locations: &FxHashSet<BoardCoordinate>,
    width: usize,
    height: usize,
) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let cur_coord = BoardCoordinate::new(x, y);
            result.push(dot_locations.contains(&cur_coord));
        }
    }

    return result;
}
