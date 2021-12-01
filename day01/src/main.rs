use day01::load;
use day01::parse_string;
use day01::puzzle_a;
use day01::puzzle_b;

fn main() {
    let contents = load();
    let depths = parse_string(&contents);

    let value = puzzle_a(&depths);
    println!("Answer to 1st question: {}", value);

    let value_b = puzzle_b(&depths);
    println!("Answer to 2nd question: {}", value_b);
}
