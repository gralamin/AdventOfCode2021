use day01::load_as_ints;
use day01::puzzle_a;
use day01::puzzle_b;

fn main() {
    let filename = "input";
    let depths = load_as_ints(filename);

    let value = puzzle_a(&depths);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&depths);
    println!("Answer to 2nd question: {}", value_b);
}
