extern crate filelib;
pub use filelib::load_no_blanks;

fn get_eplison_from_gamma(input: &str) -> String {
    let mut output: String = "".to_owned();
    for char in input.chars() {
        match char {
            '0' => output.push_str("1"),
            '1' => output.push_str("0"),
            _ => (),
        }
    }
    return output.to_string();
}

fn get_gamma_rate_as_binary_string(input: &Vec<&str>) -> String {
    // we want the most common bit in each position
    let length = input[0].chars().count();
    let mut output: String = "".to_owned();
    for n in 0..length {
        let most_common = get_most_common_bit(input, n);
        if most_common {
            output.push_str("1");
        } else {
            output.push_str("0");
        }
    }
    return output.to_string();
}

fn get_most_common_bit(input: &Vec<&str>, at: usize) -> bool {
    let mut ones = 0;
    let mut zeroes = 0;
    for cur_num in input {
        match cur_num.chars().nth(at).unwrap() {
            '0' => zeroes += 1,
            '1' => ones += 1,
            _ => {}
        }
    }
    return if ones >= zeroes { true } else { false };
}

fn recursive_filter_rating(input: &Vec<&str>, at: usize, most_common: bool) -> i32 {
    if input.len() == 1 {
        return isize::from_str_radix(input[0], 2).unwrap() as i32;
    }
    let common_bool = get_most_common_bit(input, at);
    let common_char: char;
    if most_common {
        common_char = if common_bool { '1' } else { '0' };
    } else {
        common_char = if common_bool { '0' } else { '1' };
    }
    let recurse_input: Vec<&str> = input
        .iter()
        .filter(|x| x.chars().nth(at).unwrap() == common_char)
        .cloned()
        .collect();
    return recursive_filter_rating(&recurse_input, at + 1, most_common);
}

fn get_oxygen_generator_rating(input: &Vec<&str>) -> i32 {
    return recursive_filter_rating(input, 0, true);
}

fn get_co2_scrubber_rating(input: &Vec<&str>) -> i32 {
    return recursive_filter_rating(input, 0, false);
}

/// puzzle_a
///
/// ```
/// let sample_input = vec!["00100",
///         "11110",
///         "10110",
///         "10111",
///         "10101",
///         "01111",
///         "00111",
///         "11100",
///         "10000",
///         "11001",
///         "00010",
///         "01010"];
/// assert_eq!(day03::puzzle_a(sample_input), 198);
/// ```
pub fn puzzle_a(input: Vec<&str>) -> i32 {
    let gamma_str = get_gamma_rate_as_binary_string(&input);
    let eplison_str = get_eplison_from_gamma(&gamma_str);
    let gamma = isize::from_str_radix(&gamma_str, 2).unwrap();
    let eplison = isize::from_str_radix(&eplison_str, 2).unwrap();
    return gamma as i32 * eplison as i32;
}

/// puzzle_b
///
/// ```
/// let sample_input = vec!["00100",
///         "11110",
///         "10110",
///         "10111",
///         "10101",
///         "01111",
///         "00111",
///         "11100",
///         "10000",
///         "11001",
///         "00010",
///         "01010"];
/// assert_eq!(day03::puzzle_b(sample_input), 230);
/// ```
pub fn puzzle_b(input: Vec<&str>) -> i32 {
    let oxygen = get_oxygen_generator_rating(&input);
    let scrubber = get_co2_scrubber_rating(&input);
    return oxygen * scrubber;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma() {
        let sample_input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(get_gamma_rate_as_binary_string(&sample_input), "10110");
    }

    #[test]
    fn test_eplison_from_gamma() {
        let gamma = "10110";
        assert_eq!(get_eplison_from_gamma(gamma), "01001")
    }

    #[test]
    fn test_oxygen_rating() {
        let sample_input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(get_oxygen_generator_rating(&sample_input), 23);
    }

    #[test]
    fn test_scrubber_rating() {
        let sample_input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(get_co2_scrubber_rating(&sample_input), 10);
    }
}
