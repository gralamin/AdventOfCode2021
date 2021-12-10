use day10::load;
use day10::puzzle_a;
use day10::puzzle_b;

fn main() {
    let filename = "input";
    let input = load(filename);

    let value_a = puzzle_a(&input);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(&input);
    println!("Solution to 2: {}", value_b);
}
