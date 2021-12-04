use day04::load;
use day04::parse_csv_i32_lines;
use day04::puzzle_a;
use day04::split_lines_by_blanks;
use day04::unwrap_boards;

fn main() {
    let filename = "input";
    let all_bingo_lines = load(filename);
    let partial_split: Vec<Vec<String>> = split_lines_by_blanks(&all_bingo_lines);
    let numbers_to_call = parse_csv_i32_lines(partial_split[0..1].to_vec());
    let boards = unwrap_boards(partial_split[1..].to_vec());

    let value = puzzle_a(&numbers_to_call, &boards);
    println!("Solution to 1: {}", value);
}
