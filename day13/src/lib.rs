// First question, do I use sparse maps and shrink them, or use an actual board representation?
// As I will be doing a lot of recreation, adding, and subtracting, I decice the board is more
// efficient.
pub use boardlib::BoardCoordinate;
use boardlib::{Board, BoardTraversable};
pub use rustc_hash::FxHashSet;

pub use filelib::{load, split_lines_by_blanks};
mod fold;
pub use crate::fold::Fold;

mod parse;
pub use crate::parse::{parse_coords, parse_folds};
use crate::parse::coords_to_bool;

#[derive(Debug)]
struct FoldablePaper {
    board: Board<bool>,
}

impl FoldablePaper {
    fn new(base_values: Vec<bool>, base_width: usize, base_height: usize) -> FoldablePaper {
        return FoldablePaper {
            board: Board::new(base_width, base_height, base_values),
        };
    }

    fn fold_horizontal(&self, y_to_fold: usize) -> FoldablePaper {
        let new_width = self.get_width();
        // folding takes out a row its on
        // On an equal fold, its just one half
        // If there isn't an equal fold, we should panic later for now.
        let new_height = self.get_height() / 2;
        let mut new_values: Vec<bool> = Vec::new();

        // When we fold, the distance between the y_to_fold and the y of each point remains constant
        // and X remains constant.
        for coord in self.board.coord_iter() {
            if coord.y <= y_to_fold {
                // skip over these for now.
                continue;
            } else {
                let distance = coord.y - y_to_fold;
                let matching_coord = BoardCoordinate::new(coord.x, y_to_fold - distance);
                if let Some(unfolded_v) = self.board.get_value(matching_coord) {
                    if let Some(folded_v) = self.board.get_value(coord) {
                        new_values.push(unfolded_v || folded_v);
                    } else {
                        panic!("Couldn't find folded value I'm on");
                    }
                } else {
                    panic!("Couldn't find unfolded value - might be unequal fold");
                }
            }
        }

        return FoldablePaper::new(new_values, new_width, new_height);
    }

    fn fold_vertical(&self, x_to_fold: usize) -> FoldablePaper {
        // folding takes out a col its on
        // On an equal fold, its just one half
        // If there isn't an equal fold, we should panic later for now.
        let new_width = self.get_width() / 2;
        let new_height = self.get_height();
        let mut new_values: Vec<bool> = Vec::new();

        // When we fold, the distance between the x_to_fold and the x of each point remains constant
        // and y remains constant.
        for coord in self.board.coord_iter() {
            if coord.x <= x_to_fold {
                // skip over these for now.
                continue;
            } else {
                let distance = coord.x - x_to_fold;
                let matching_coord = BoardCoordinate::new(x_to_fold - distance, coord.y);
                if let Some(unfolded_v) = self.board.get_value(matching_coord) {
                    if let Some(folded_v) = self.board.get_value(coord) {
                        new_values.push(unfolded_v || folded_v);
                    } else {
                        panic!("Couldn't find folded value I'm on");
                    }
                } else {
                    panic!("Couldn't find unfolded value - might be unequal fold");
                }
            }
        }

        return FoldablePaper::new(new_values, new_width, new_height);
    }

    fn get_num_dots(&self) -> usize {
        let mut count: usize = 0;
        for coord in self.board.coord_iter() {
            if let Some(value) = self.board.get_value(coord) {
                if value {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn get_width(&self) -> usize {
        return self.board.get_width();
    }

    fn get_height(&self) -> usize {
        return self.board.get_height();
    }

    fn fancy_print(&self) -> String {
        let mut string_parts: Vec<&str> = Vec::new();
        let mut last_y = 0;
        for coord in self.board.coord_iter() {
            if coord.y > last_y {
                string_parts.push("\n");
            }
            last_y = coord.y;
            if self.board.get_value(coord).unwrap() {
                string_parts.push("#");
            } else {
                string_parts.push(" ");
            }
        }
        return string_parts
            .iter()
            .map(|x| *x)
            .collect::<String>()
            .chars() // Flip it, note chars.rev can go wrong in unicode cases.
            .rev()
            .collect::<String>();
    }
}

/// Perform the first fold and count the dots
///
/// ```
/// use boardlib::BoardCoordinate;
/// use rustc_hash::FxHashSet;
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
/// let folds = vec![day13::Fold::Horizontal(7), day13::Fold::Vertical(5)];
/// assert_eq!(day13::puzzle_a(&dot_coords, &folds), 17);
/// ```
pub fn puzzle_a(coords: &FxHashSet<BoardCoordinate>, folds: &Vec<Fold>) -> usize {
    let mut width: usize = 0;
    let mut height: usize = 0;
    for v in coords.iter() {
        if v.x > width {
            width = v.x;
        }
        if v.y > height {
            height = v.y;
        }
    }
    // Width and height are 1 indexed, not 0 indexed.
    width += 1;
    height += 1;

    let values = coords_to_bool(coords, width, height);
    let paper = FoldablePaper::new(values, width, height);
    let folded_paper: FoldablePaper;
    if let Some(fold) = folds.first() {
        match fold {
            Fold::Vertical(a) => folded_paper = paper.fold_vertical(*a),
            Fold::Horizontal(a) => folded_paper = paper.fold_horizontal(*a),
        }
    } else {
        return 0;
    }
    return folded_paper.get_num_dots();
}

/// Perform all the folds, then figure out what letters there are
///
/// ```
/// use boardlib::BoardCoordinate;
/// use rustc_hash::FxHashSet;
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
/// let folds = vec![day13::Fold::Horizontal(7), day13::Fold::Vertical(5)];
/// assert_eq!(day13::puzzle_b(&dot_coords, &folds), "#####\n#   #\n#   #\n#   #\n#####\n     \n     ");
/// ```
pub fn puzzle_b(coords: &FxHashSet<BoardCoordinate>, folds: &Vec<Fold>) -> String {
    let mut width: usize = 0;
    let mut height: usize = 0;
    for v in coords.iter() {
        if v.x > width {
            width = v.x;
        }
        if v.y > height {
            height = v.y;
        }
    }
    // Width and height are 1 indexed, not 0 indexed.
    width += 1;
    height += 1;

    let values = coords_to_bool(coords, width, height);
    let paper = FoldablePaper::new(values, width, height);
    let mut folded_paper: FoldablePaper = paper;
    for fold in folds {
        match fold {
            Fold::Vertical(a) => folded_paper = folded_paper.fold_vertical(*a),
            Fold::Horizontal(a) => folded_paper = folded_paper.fold_horizontal(*a),
        }
    }

    // I'm too lazy to build an OCR that can flip letters if they are upside down...
    return folded_paper.fancy_print();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_paper() -> FoldablePaper {
        let height = 15;
        let width = 11;
        let mut dot_coords: FxHashSet<BoardCoordinate> = FxHashSet::default();
        dot_coords.insert(BoardCoordinate::new(6, 10));
        dot_coords.insert(BoardCoordinate::new(0, 14));
        dot_coords.insert(BoardCoordinate::new(9, 10));
        dot_coords.insert(BoardCoordinate::new(0, 3));
        dot_coords.insert(BoardCoordinate::new(10, 4));
        dot_coords.insert(BoardCoordinate::new(4, 11));
        dot_coords.insert(BoardCoordinate::new(6, 0));
        dot_coords.insert(BoardCoordinate::new(6, 12));
        dot_coords.insert(BoardCoordinate::new(4, 1));
        dot_coords.insert(BoardCoordinate::new(0, 13));
        dot_coords.insert(BoardCoordinate::new(10, 12));
        dot_coords.insert(BoardCoordinate::new(3, 4));
        dot_coords.insert(BoardCoordinate::new(3, 0));
        dot_coords.insert(BoardCoordinate::new(8, 4));
        dot_coords.insert(BoardCoordinate::new(1, 10));
        dot_coords.insert(BoardCoordinate::new(2, 14));
        dot_coords.insert(BoardCoordinate::new(8, 10));
        dot_coords.insert(BoardCoordinate::new(9, 0));
        let values = coords_to_bool(&dot_coords, width, height);
        return FoldablePaper::new(values, width, height);
    }

    #[test]
    fn test_get_num_dots() {
        let paper = make_paper();
        assert_eq!(paper.get_num_dots(), 18);
    }

    #[test]
    fn test_fold_vertically() {
        let paper = make_paper();
        let folded_paper = paper.fold_horizontal(7);
        assert_eq!(folded_paper.get_num_dots(), 17);
        assert_eq!(folded_paper.get_width(), 11);
        assert_eq!(folded_paper.get_height(), 7);
    }

    #[test]
    fn test_fold_vertically_then_horizontally() {
        let paper = make_paper();
        let folded_paper = paper.fold_horizontal(7);
        let twice_folded = folded_paper.fold_vertical(5);
        assert_eq!(twice_folded.get_num_dots(), 16);
        assert_eq!(twice_folded.get_width(), 5);
        assert_eq!(twice_folded.get_height(), 7);
    }
}
