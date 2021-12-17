use day17::load;
use day17::load_target_area;
use day17::puzzle_a;
use day17::puzzle_b;

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let (x1, y1, x2, y2) = load_target_area(&all_lines);

    let value_a = puzzle_a(x1, y1, x2, y2);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(x1, y1, x2, y2);
    println!("Solution to 2: {}", value_b);
}
