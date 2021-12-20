use day20::puzzle_a;
use day20::puzzle_b;
use day20::{load, split_lines_by_blanks};
use day20::{parse_image, parse_image_enhancement_algorithm};

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let split = split_lines_by_blanks(&all_lines);

    let image_enhancement = parse_image_enhancement_algorithm(&split[0][0]);
    let image = parse_image(&split[1]);
    let value_a = puzzle_a(&image_enhancement, &image);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(&image_enhancement, &image);
    println!("Solution to 2: {}", value_b);
}
