use day18::load;
use day18::parse;
use day18::puzzle_a;
//use day18::puzzle_b;

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let parsed = parse(&all_lines);
    
    let value_a = puzzle_a(&parsed);
    println!("Solution to 1: {}", value_a);

    // let value_b = puzzle_b(&parsed);
    // println!("Solution to 2: {}", value_b);
}
