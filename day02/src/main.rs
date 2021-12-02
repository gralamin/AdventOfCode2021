use day02::directional_commands_from_strs;
use day02::load_no_blanks;
use day02::puzzle_a;
use day02::puzzle_b;

fn main() {
    let filename = "input";
    let command_strs = load_no_blanks(filename);
    let command_refs: Vec<&str> = command_strs.iter().map(AsRef::as_ref).collect();
    let commands = directional_commands_from_strs(command_refs);

    let value_a = puzzle_a(commands);
    println!("Answer to 1st question: {}", value_a);

    let command_refs_b: Vec<&str> = command_strs.iter().map(AsRef::as_ref).collect();
    let commands_b = directional_commands_from_strs(command_refs_b);

    let value_b = puzzle_b(commands_b);
    println!("Answer to 2nd question: {}", value_b);
}
