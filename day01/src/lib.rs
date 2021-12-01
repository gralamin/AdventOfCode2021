use std::fs;

/// Load the "input" file
pub fn load() -> String {
    let contents = fs::read_to_string("input").expect("Something went wrong reading");
    return contents;
}

/// parse A string, including empty lines, to a list of integers
///
/// This filters out empty lines, and returns a vector of i32s.
///
/// ```
/// assert_eq!(day01::parse_string("1\n2"), vec![1, 2]);
/// ```
pub fn parse_string(text_input: &str) -> Vec<i32> {
    let no_empty_lines: Vec<&str> = text_input
        .lines()
        .filter(|&s| !s.is_empty() && !s.trim().is_empty())
        .collect::<Vec<_>>();
    let result: Vec<i32> = no_empty_lines
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    return result;
}

/// Get the number of times depths increases
/// ```
/// let vec1 = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
/// assert_eq!(day01::puzzle_a(vec1), 7);
/// ```
/// This has to be in here, due to how rust doctests work...
pub fn puzzle_a(depths: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut last_value = depths[0];
    for x in depths {
        if x > last_value {
            count += 1;
        }
        last_value = x;
    }
    return count;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_string() {
        let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
        let expected = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = parse_string(input);
        assert_eq!(result, expected);
    }
}
