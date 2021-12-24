use day24::load;
use day24::parse_all_instructions;
use day24::puzzle_a;
//use day24::puzzle_b;

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let instructions = parse_all_instructions(&all_lines);

    let value_a = puzzle_a(&instructions);
    println!("Solution to 1: {}", value_a);

    //let value_b = puzzle_b(&instructions);
    //println!("Solution to 2: {}", value_b);
}
