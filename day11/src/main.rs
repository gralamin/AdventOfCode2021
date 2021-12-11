use day11::extract_all_nums;
use day11::load;
use day11::puzzle_a;
//use day11::puzzle_b;

fn main() {
    let filename = "input";
    let input = load(filename);
    let numbers = extract_all_nums(&input);

    let value_a = puzzle_a(&numbers);
    println!("Solution to 1: {}", value_a);
}
