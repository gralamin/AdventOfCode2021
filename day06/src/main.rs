use day06::load;
use day06::parse_csv_i32_lines;


fn main() {
    let data = load("input");
    let input_strings: Vec<Vec<String>> = vec![data.lines().map(|s| s.to_string()).collect()];
    let input_ints: Vec<i32> = parse_csv_i32_lines(input_strings);
}
