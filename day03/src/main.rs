//use day03::directional_commands_from_strs;
use day03::load_no_blanks;
use day03::puzzle_a;
use day03::puzzle_b;

fn main() {
    let filename = "input";
    let diagnostic_strs = load_no_blanks(filename);
    let diagnostic_refs: Vec<&str> = diagnostic_strs.iter().map(AsRef::as_ref).collect();

    let value_a = puzzle_a(diagnostic_refs);
    println!("Answer to 1st question: {}", value_a);

    /*
    let command_refs_b: Vec<&str> = command_strs.iter().map(AsRef::as_ref).collect();
    let commands_b = directional_commands_from_strs(command_refs_b);

    let value_b = puzzle_b(commands_b);
    println!("Answer to 2nd question: {}", value_b);
    */
}
