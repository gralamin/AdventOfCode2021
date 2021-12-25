use day25::load;
use day25::parse_sea_cucs;
use day25::puzzle_a;
//use day24::puzzle_b;
//use day24::Cache;

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let cucumbers = parse_sea_cucs(&all_lines);

    assert_eq!(cucumbers.len(), 137);
    assert_eq!(cucumbers[0].len(), 139);

    //let mut cache = Cache::default();

    let value_a = puzzle_a(&cucumbers);
    println!("Solution to 1: {}", value_a);

    // let value_b = puzzle_b(&cucumbers);
    // println!("Solution to 2: {}", value_b);
}
