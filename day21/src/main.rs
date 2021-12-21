use day21::puzzle_a;
//use day21::puzzle_b;
use day21::load;
use day21::parse_player_pos;

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let player_pos = parse_player_pos(&all_lines);

    let value_a = puzzle_a(&player_pos);
    println!("Solution to 1: {}", value_a);

    //let value_b = puzzle_b(&player_pos);
    //println!("Solution to 2: {}", value_b);
}
