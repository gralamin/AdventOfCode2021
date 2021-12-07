use day07::load;
use day07::parse_csv_i32_lines;
use day07::puzzle_a;
//use day07::puzzle_b;

fn main() {
    let data = load("input");
    let input_strings: Vec<Vec<String>> = vec![data.lines().map(|s| s.to_string()).collect()];
    let input_ints: Vec<i32> = parse_csv_i32_lines(input_strings);

    let value_a = puzzle_a(&input_ints);
    println!("Solution to puzzle 1: {}", value_a);
}
