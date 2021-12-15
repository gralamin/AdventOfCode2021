use day15::load;
use day15::puzzle_a;
use day15::puzzle_b;

fn main() {
    let filename = "input";
    let all_lines = load(filename);

    let value_a = puzzle_a(&all_lines);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(&all_lines);
    println!("Solution to 2: {}", value_b);
}
