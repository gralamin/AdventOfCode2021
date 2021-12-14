use day14::puzzle_a;
use day14::{load, split_lines_by_blanks};
use day14::puzzle_b;
use day14::{create_rules, FxHashMap, PolyPair};
use day14::{get_alphabet, FxHashSet};

fn main() {
    let filename = "input";
    let all_lines = load(filename);
    let sections_split = split_lines_by_blanks(&all_lines);
    let template: String;
    let rules: FxHashMap<PolyPair, String>;
    let alphabet: FxHashSet<String>;

    let template_section = sections_split.first().unwrap();
    template = template_section.first().unwrap().to_string();

    let rules_section = sections_split.last().unwrap();
    rules = create_rules(rules_section.to_vec());

    alphabet = get_alphabet(&template, &rules);

    let value_a = puzzle_a(&template, &rules, &alphabet);
    println!("Solution to 1: {}", value_a);

    let value_b = puzzle_b(&template, &rules, &alphabet);
    println!("Solution to 2: {}", value_b);
}
