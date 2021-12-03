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
    let mut ones = 0;
    let mut zeroes = 0;
    for n in 0..length {
        ones = 0;
        zeroes = 0;
        for cur_num in input {
            match cur_num.chars().nth(n).unwrap() {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => {}
            }
        }
        if ones > zeroes {
            output.push_str("1");
        } else {
            output.push_str("0");
        }
    }
    return output.to_string();
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
pub fn puzzle_b() {}

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

    fn test_eplison_from_gamma() {
        let gamma = "10110";
        assert_eq!(get_eplison_from_gamma(gamma), "01001")
    }
}
