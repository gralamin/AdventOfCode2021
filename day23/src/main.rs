use day23::load;
use day23::parse_amphipod;
use day23::puzzle_a;
use day23::puzzle_b;

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let amphipod = parse_amphipod(&all_lines);

    let value_a = puzzle_a(&amphipod);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(&amphipod);
    println!("Solution to 2: {}", value_b);
}
