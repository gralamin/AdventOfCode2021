use day05::load_no_blanks;
use day05::parse_line_to_linecoords;
use day05::puzzle_a;
use day05::puzzle_b;

fn main() {
    let filename = "input";
    let all_lines = load_no_blanks(filename);
    let pos_pairs: Vec<(i32, i32, i32, i32)> = all_lines
        .iter()
        .map(|s| parse_line_to_linecoords(s))
        .collect();

    let value_a = puzzle_a(&pos_pairs);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(&pos_pairs);
    println!("Solution to 1: {}", value_b);
}
