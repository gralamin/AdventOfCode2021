use day19::parse_scanner;
use day19::puzzle_a;
use day19::{load, split_lines_by_blanks};
use day19::{FxHashMap, IVec3};
use day19::puzzle_b;

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let split = split_lines_by_blanks(&all_lines);
    let scanner_map: FxHashMap<usize, Vec<IVec3>> =
        split.iter().map(|x| parse_scanner(x)).collect();

    let value_a = puzzle_a(&scanner_map);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(&scanner_map);
    println!("Solution to 2: {}", value_b);
}
