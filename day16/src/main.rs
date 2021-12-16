use day16::load;
use day16::parse_hexadecimal;
use day16::puzzle_a;
//use day16::puzzle_b;

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let packet_stream = parse_hexadecimal(&all_lines);

    let value_a = puzzle_a(&packet_stream);
    println!("Solution to 1: {}", value_a);

    //let value_b = puzzle_b(&all_lines);
    //println!("Solution to 2: {}", value_b);
}
