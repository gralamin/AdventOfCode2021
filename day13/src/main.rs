use day13::load;
use day13::parse_coords;
use day13::parse_folds;
use day13::puzzle_a;
use day13::puzzle_b;
use day13::split_lines_by_blanks;
use day13::Fold;
use day13::FxHashSet;

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let sections_split = split_lines_by_blanks(&all_lines);
    let mut coords = FxHashSet::default();
    let mut folds: Vec<Fold> = Vec::new();

    if let Some(coord_section) = sections_split.first() {
        coords = parse_coords(coord_section);
    }

    if let Some(fold_section) = sections_split.last() {
        folds = parse_folds(fold_section);
    }

    let value_a = puzzle_a(&coords, &folds);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(&coords, &folds);
    println!("Solution to 2: {}", value_b);
}
