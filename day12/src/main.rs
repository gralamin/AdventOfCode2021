use day12::load_no_blanks;
use day12::puzzle_a;
//use day12::puzzle_b;

fn main() {
    let filename = "input";
    let all_lines = load_no_blanks(filename);

    let value_a = puzzle_a(&all_lines);
    println!("Solution to 1: {}", value_a);

    //let value_b = puzzle_b(&all_lines);
    //println!("Solution to 2: {}", value_b);
}
