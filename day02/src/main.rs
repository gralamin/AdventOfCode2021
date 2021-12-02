use day02::directional_commands_from_strs;
use day02::load_no_blanks;
use day02::puzzle_a;

fn main() {
    let filename = "input";
    let command_strs = load_no_blanks(filename);
    let command_refs: Vec<&str> = command_strs.iter().map(AsRef::as_ref).collect();
    let commands = directional_commands_from_strs(command_refs);

    let value_a = puzzle_a(commands);
    println!("Answer to 1st question: {}", value_a);
}
