extern crate filelib;
pub use filelib::load_no_blanks;

const ONE: bool = true;
const ZERO: bool = false;

trait BitExtractable {
    fn get_bit(&self, at: i32) -> bool;
    fn get_bit_from_left(&self, at: i32) -> bool;
}

#[derive(Debug)]
struct BinaryString {
    length: i32,
    value: i32,
}

fn build_binary_string(s: &str) -> BinaryString {
    return BinaryString {
        length: s.chars().count().try_into().unwrap(),
        value: isize::from_str_radix(s, 2).unwrap() as i32,
    };
}

impl BitExtractable for BinaryString {
    fn get_bit(&self, at: i32) -> bool {
        if at < self.length {
            return self.value & (1 << at) != 0;
        } else {
            return ZERO;
        }
    }

    fn get_bit_from_left(&self, at: i32) -> bool {
        return self.get_bit(self.length - (at + 1));
    }
}

impl Clone for BinaryString {
    fn clone(&self) -> Self {
        return BinaryString {
            length: self.length,
            value: self.value,
        };
    }
}

fn to_bin_strings(inputs: &Vec<&str>) -> Vec<BinaryString> {
    return inputs.iter().map(|s| build_binary_string(s)).collect();
}

fn get_eplison_from_gamma(input: BinaryString) -> BinaryString {
    let length = input.length;
    let mut value = 0;
    let base: i32 = 2;

    for n in (0..length).rev() {
        if input.get_bit(n) == ZERO {
            value += base.pow(n.try_into().unwrap());
        }
    }
    return BinaryString {
        length: length,
        value: value,
    };
}

fn find_gamma_rate(inputs: &Vec<BinaryString>) -> BinaryString {
    let length = inputs[0].length;
    let mut value: i32 = 0;
    let base: i32 = 2;
    for n in (0..length).rev() {
        let most_common = get_most_common_bit(inputs, n);
        if most_common == ONE {
            value += base.pow(n.try_into().unwrap());
        }
    }

    return BinaryString {
        length: length,
        value: value,
    };
}

fn get_most_common_bit(inputs: &Vec<BinaryString>, at: i32) -> bool {
    let mut ones = 0;
    let mut zeroes = 0;
    for cur in inputs {
        match cur.get_bit(at) {
            ONE => ones += 1,
            ZERO => zeroes += 1,
        }
    }
    return if ones >= zeroes { ONE } else { ZERO };
}

fn recursive_filter_rating(input: &Vec<BinaryString>, at: usize, most_common: bool) -> i32 {
    if input.len() == 0 {
        // something went wrong.
        return -999;
    }
    if input.len() == 1 {
        return input[0].value;
    }
    let common_bool = get_most_common_bit(input, at.try_into().unwrap());
    let check_for: bool;
    if most_common {
        check_for = if common_bool { ONE } else { ZERO };
    } else {
        check_for = if common_bool { ZERO } else { ONE };
    }
    let recurse_input: Vec<BinaryString> = input
        .into_iter()
        .filter(|x| x.get_bit(at.try_into().unwrap()) == check_for)
        .cloned()
        .collect();
    let next_at: usize;
    if at == 0 {
        next_at = 0;
    } else {
        next_at = at - 1;
    }
    return recursive_filter_rating(&recurse_input, next_at, most_common);
}

fn get_oxygen_generator_rating(input: &Vec<BinaryString>) -> i32 {
    return recursive_filter_rating(input, (input[0].length - 1).try_into().unwrap(), true);
}

fn get_co2_scrubber_rating(input: &Vec<BinaryString>) -> i32 {
    return recursive_filter_rating(input, (input[0].length - 1).try_into().unwrap(), false);
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
    let bin_strings = to_bin_strings(&input);
    let gamma = find_gamma_rate(&bin_strings);
    let gamma_v = gamma.value;
    let eplison = get_eplison_from_gamma(gamma);
    return gamma_v * eplison.value;
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
    let bin_strings = to_bin_strings(&input);
    let oxygen = get_oxygen_generator_rating(&bin_strings);
    let scrubber = get_co2_scrubber_rating(&bin_strings);
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
        let sample = to_bin_strings(&sample_input);
        assert_eq!(find_gamma_rate(&sample).value, 22);
    }

    #[test]
    fn test_eplison_from_gamma() {
        let gamma = build_binary_string("10110");
        assert_eq!(get_eplison_from_gamma(gamma).value, 9)
    }

    #[test]
    fn test_oxygen_rating() {
        let sample_input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let sample = to_bin_strings(&sample_input);
        assert_eq!(get_oxygen_generator_rating(&sample), 23);
    }

    #[test]
    fn test_scrubber_rating() {
        let sample_input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let sample = to_bin_strings(&sample_input);
        assert_eq!(get_co2_scrubber_rating(&sample), 10);
    }
}
