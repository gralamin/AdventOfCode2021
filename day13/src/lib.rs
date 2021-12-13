// First question, do I use sparse maps and shrink them, or use an actual board representation?
// As I will be doing a lot of recreation, adding, and subtracting, I decice the board is more
// efficient.
pub use boardlib::BoardCoordinate;
pub use rustc_hash::FxHashSet;

pub use filelib::{load, split_lines_by_blanks};
mod fold;
pub use crate::fold::Fold;

mod parse;
use crate::parse::coords_to_bool;
pub use crate::parse::{parse_coords, parse_folds};

mod foldable;
use crate::foldable::FoldablePaper;

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
/// assert_eq!(day13::puzzle_b(&dot_coords, &folds), "█████\n█   █\n█   █\n█   █\n█████\n     \n     ");
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
