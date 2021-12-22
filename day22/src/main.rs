use day22::load;
use day22::parse_instructions;
use day22::puzzle_a;
// use day22::puzzle_b;

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let instructions = parse_instructions(&all_lines);

    let value_a = puzzle_a(&instructions);
    println!("Solution to 1: {}", value_a);

    // let value_b = puzzle_b(&instructions);
    // println!("Solution to 2: {}", value_b);
}
