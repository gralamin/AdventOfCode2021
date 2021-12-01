use std::fs;

/// Load the "input" file
pub fn load(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading");
    return contents;
}

/// remove blank lines
fn remove_blanks(text_input: &str) -> Vec<String> {
    return text_input
        .lines()
        .filter(|&s| !s.is_empty() && !s.trim().is_empty())
        .map(str::to_string)
        .collect();
}

/// Load without blank lines
pub fn load_no_blanks(filename: &str) -> Vec<String> {
    return remove_blanks(&load(filename));
}

fn strings_to_i32(strings: Vec<&str>) -> Vec<i32> {
    let result: Vec<i32> = strings.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    return result;
}

/// Load and convert to 32-bit integers
pub fn load_as_ints(filename: &str) -> Vec<i32> {
    let strings = load_no_blanks(filename);
    return strings_to_i32(strings.iter().map(AsRef::as_ref).collect());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_blanks() {
        let input = "\n199\n200\n208\n210\n\n200\n207\n240\n269\n260\n263\n";
        let expected = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        assert_eq!(remove_blanks(input), expected);
    }

    #[test]
    fn test_strings_to_i32() {
        let input = vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ];
        let expected = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(strings_to_i32(input), expected);
    }
}
