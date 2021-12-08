use day08::load_no_blanks;
use day08::puzzle_a;
use day08::puzzle_b;
use day08::split_input;

fn main() {
    let filename = "input";
    let all_lines = load_no_blanks(filename);
    let split_tuple = split_input(all_lines);
    let signals = split_tuple.0;
    let values = split_tuple.1;

    let value_a = puzzle_a(&signals, &values);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(&signals, &values);
    println!("Solution to 2: {}", value_b);
}
