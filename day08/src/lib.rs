extern crate filelib;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

pub use filelib::load_no_blanks;

/// Split the input by the | into two vectors of equal size
///
/// ```
/// let input = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
/// "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
/// "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
/// "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
/// "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
/// "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
/// "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
/// "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
/// "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
/// "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string()
/// ];
/// let expected_signals = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb",
/// "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec",
/// "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef",
/// "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega",
/// "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga",
/// "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf",
/// "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf",
/// "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd",
/// "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg",
/// "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc"
/// ];
/// let expected_output_values = vec!["fdgacbe cefdb cefbgd gcbe",
/// "fcgedb cgb dgebacf gc",
/// "cg cg fdcagb cbg",
/// "efabcd cedba gadfec cb",
/// "gecf egdcabf bgf bfgea",
/// "gebdcfa ecba ca fadegcb",
/// "cefg dcbef fcge gbcadfe",
/// "ed bcgafe cdgba cbgef",
/// "gbdfcae bgc cg cgb",
/// "fgae cfgab fg bagce"
/// ];
/// let result_tuple = day08::split_input(input);
/// assert_eq!(result_tuple.0, expected_signals);
/// assert_eq!(result_tuple.1, expected_output_values);
/// ```
pub fn split_input(input_lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut signals: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();
    for line in input_lines {
        let mut splitter = line.split("|");
        let cur_signals = splitter.next().unwrap();
        let cur_values = splitter.next().unwrap();
        signals.push(cur_signals.trim().to_string());
        values.push(cur_values.trim().to_string());
    }
    return (signals, values);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl std::str::FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "a" => Ok(Segment::A),
            "b" => Ok(Segment::B),
            "c" => Ok(Segment::C),
            "d" => Ok(Segment::D),
            "e" => Ok(Segment::E),
            "f" => Ok(Segment::F),
            "g" => Ok(Segment::G),
            _ => Err(()),
        };
    }
}

fn convert_str_to_segment_vec(input: &str) -> Vec<Segment> {
    return input
        .chars()
        .map(|c| c.to_string().parse::<Segment>().unwrap())
        .collect();
}

fn convert_line_to_segment_vec(line: &str) -> Vec<Vec<Segment>> {
    return line
        .split(" ")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| convert_str_to_segment_vec(x))
        .collect();
}

fn get_default_displays() -> FxHashMap<i32, Vec<Segment>> {
    let mut map: FxHashMap<i32, Vec<Segment>> = FxHashMap::default();

    map.insert(
        0,
        vec![
            Segment::A,
            Segment::B,
            Segment::C,
            Segment::E,
            Segment::F,
            Segment::G,
        ],
    );
    map.insert(1, vec![Segment::C, Segment::F]);
    map.insert(
        2,
        vec![Segment::A, Segment::C, Segment::D, Segment::E, Segment::G],
    );
    map.insert(
        3,
        vec![Segment::A, Segment::C, Segment::D, Segment::F, Segment::G],
    );
    map.insert(4, vec![Segment::B, Segment::C, Segment::D, Segment::F]);
    map.insert(
        5,
        vec![Segment::A, Segment::B, Segment::D, Segment::F, Segment::G],
    );
    map.insert(
        6,
        vec![
            Segment::A,
            Segment::B,
            Segment::D,
            Segment::E,
            Segment::F,
            Segment::G,
        ],
    );
    map.insert(7, vec![Segment::A, Segment::C, Segment::F]);
    map.insert(
        8,
        vec![
            Segment::A,
            Segment::B,
            Segment::C,
            Segment::D,
            Segment::E,
            Segment::F,
            Segment::G,
        ],
    );
    map.insert(
        9,
        vec![
            Segment::A,
            Segment::B,
            Segment::C,
            Segment::D,
            Segment::F,
            Segment::G,
        ],
    );

    return map;
}

/// Use logic to figure out which has to be which
fn deduce_map(
    input_line: &Vec<Vec<Segment>>,
    default_displays: &FxHashMap<i32, Vec<Segment>>,
) -> FxHashMap<Segment, Segment> {
    let mut segment_to_segment_map: FxHashMap<Segment, Segment> = FxHashMap::default();

    // TODO: Instead of manual logic, can I get the algorithm to figure it out without my help?

    // Numbers to possible segments for that number.
    let mut possible_segments: FxHashMap<i32, FxHashSet<Segment>> = FxHashMap::default();
    possible_segments.insert(0, FxHashSet::default());
    possible_segments.insert(1, FxHashSet::default());
    possible_segments.insert(2, FxHashSet::default());
    possible_segments.insert(3, FxHashSet::default());
    possible_segments.insert(4, FxHashSet::default());
    possible_segments.insert(5, FxHashSet::default());
    possible_segments.insert(6, FxHashSet::default());
    possible_segments.insert(7, FxHashSet::default());
    possible_segments.insert(8, FxHashSet::default());
    possible_segments.insert(9, FxHashSet::default());

    let mut used_segments: FxHashSet<Segment> = FxHashSet::default();

    // 1, 4, 7, and 8 can be deduced via the length only
    let one_length = default_displays.get(&1).unwrap().len();
    let four_length = default_displays.get(&4).unwrap().len();
    let seven_length = default_displays.get(&7).unwrap().len();
    let eight_length = default_displays.get(&8).unwrap().len();
    for input_group in input_line.iter() {
        if input_group.len() == one_length {
            for x in input_group {
                possible_segments
                    .entry(1)
                    .or_insert(FxHashSet::default())
                    .insert(*x);
            }
        }
        if input_group.len() == four_length {
            for x in input_group {
                possible_segments
                    .entry(4)
                    .or_insert(FxHashSet::default())
                    .insert(*x);
            }
        }
        if input_group.len() == seven_length {
            for x in input_group {
                possible_segments
                    .entry(7)
                    .or_insert(FxHashSet::default())
                    .insert(*x);
            }
        }
        if input_group.len() == eight_length {
            for x in input_group {
                possible_segments
                    .entry(8)
                    .or_insert(FxHashSet::default())
                    .insert(*x);
            }
        }
    }

    // solve a
    // 7 is 1 + a
    let seven_difference = possible_segments.get(&7).unwrap() - possible_segments.get(&1).unwrap();
    for a_seg in seven_difference {
        segment_to_segment_map.insert(Segment::A, a_seg);
        used_segments.insert(a_seg);
    }

    // Next, solve c and f
    // 0, 6, 9 have 6 segments
    // But all three include f (in 1)
    // so if we see 6 segments, but don't have a value from one, that must be c
    // which also gives us f by default.
    let zero_length = default_displays.get(&0).unwrap().len();
    let six_length = default_displays.get(&6).unwrap().len();
    let _nine_length = default_displays.get(&9).unwrap().len();

    for input_group in input_line.iter() {
        if input_group.len() == six_length {
            // This must be c
            if !input_group.contains(possible_segments.get(&1).unwrap().iter().nth(0).unwrap()) {
                for x in input_group {
                    possible_segments
                        .entry(6)
                        .or_insert(FxHashSet::default())
                        .insert(*x);
                }
                segment_to_segment_map.insert(
                    Segment::C,
                    *possible_segments.get(&1).unwrap().iter().nth(0).unwrap(),
                );
                segment_to_segment_map.insert(
                    Segment::F,
                    *possible_segments.get(&1).unwrap().iter().nth(1).unwrap(),
                );
                used_segments.insert(*possible_segments.get(&1).unwrap().iter().nth(0).unwrap());
                used_segments.insert(*possible_segments.get(&1).unwrap().iter().nth(1).unwrap());
            } else if !input_group
                .contains(possible_segments.get(&1).unwrap().iter().nth(1).unwrap())
            {
                for x in input_group {
                    possible_segments
                        .entry(6)
                        .or_insert(FxHashSet::default())
                        .insert(*x);
                }
                segment_to_segment_map.insert(
                    Segment::C,
                    *possible_segments.get(&1).unwrap().iter().nth(1).unwrap(),
                );
                segment_to_segment_map.insert(
                    Segment::F,
                    *possible_segments.get(&1).unwrap().iter().nth(0).unwrap(),
                );
                used_segments.insert(*possible_segments.get(&1).unwrap().iter().nth(0).unwrap());
                used_segments.insert(*possible_segments.get(&1).unwrap().iter().nth(1).unwrap());
            }
        }
    }

    // Get some of 2, 3, and 5 info
    // 2 has c but is missing f
    // 3 has c and f
    // 5 has f but not c
    let two_length = default_displays.get(&2).unwrap().len();
    let _three_length = default_displays.get(&3).unwrap().len();
    let _five_length = default_displays.get(&5).unwrap().len();

    for input_group in input_line.iter() {
        if input_group.len() == two_length {
            if input_group.contains(segment_to_segment_map.get(&Segment::C).unwrap())
                && input_group.contains(segment_to_segment_map.get(&Segment::F).unwrap())
            {
                // This is 3
                for x in input_group {
                    possible_segments
                        .entry(3)
                        .or_insert(FxHashSet::default())
                        .insert(*x);
                }
            } else if input_group.contains(segment_to_segment_map.get(&Segment::F).unwrap()) {
                // Have an f, so can't but 2, must be 5
                for x in input_group {
                    possible_segments
                        .entry(5)
                        .or_insert(FxHashSet::default())
                        .insert(*x);
                }
            } else if input_group.contains(segment_to_segment_map.get(&Segment::C).unwrap()) {
                // This is 2
                for x in input_group {
                    possible_segments
                        .entry(2)
                        .or_insert(FxHashSet::default())
                        .insert(*x);
                }
            }
        }
    }

    // Solve b / e
    // 2 has e, but not b and f
    // 5 has b, but not c or e
    // since c and f are known, we can solve for b and e.
    for two_segment in possible_segments.get(&2).unwrap().iter() {
        if two_segment != segment_to_segment_map.get(&Segment::C).unwrap() {
            if !possible_segments.get(&5).unwrap().contains(two_segment) {
                segment_to_segment_map.insert(Segment::E, *two_segment);
                used_segments.insert(*two_segment);
            }
        }
    }

    for five_segment in possible_segments.get(&5).unwrap().iter() {
        if five_segment != segment_to_segment_map.get(&Segment::F).unwrap() {
            if !possible_segments.get(&2).unwrap().contains(five_segment) {
                segment_to_segment_map.insert(Segment::B, *five_segment);
                used_segments.insert(*five_segment);
            }
        }
    }

    // Now that we have c and e, we can find whats 0 and 9
    for input_group in input_line.iter() {
        if input_group.len() == zero_length {
            if !input_group.contains(segment_to_segment_map.get(&Segment::E).unwrap()) {
                for x in input_group {
                    possible_segments
                        .entry(9)
                        .or_insert(FxHashSet::default())
                        .insert(*x);
                }
            } else if input_group.contains(segment_to_segment_map.get(&Segment::C).unwrap()) {
                for x in input_group {
                    possible_segments
                        .entry(0)
                        .or_insert(FxHashSet::default())
                        .insert(*x);
                }
            }
        }
    }

    // Almost done just D and G!
    // Whatever isn't in 0 is d.
    // all elements is in the array for 8
    for segment in default_displays.get(&8).unwrap().iter() {
        if !possible_segments.get(&0).unwrap().contains(segment) {
            segment_to_segment_map.insert(Segment::D, *segment);
            used_segments.insert(*segment);
            break;
        }
    }

    // Finally g
    // We know everything else, so this is the remaining one.
    for segment in default_displays.get(&8).unwrap().iter() {
        if !used_segments.contains(segment) {
            segment_to_segment_map.insert(Segment::G, *segment);
        }
    }

    return segment_to_segment_map;
}

/// Get the number of times the scrambled digits 1, 4, 7, or 8 show up
///
/// ```
/// let signals = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb".to_string(),
/// "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec".to_string(),
/// "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef".to_string(),
/// "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega".to_string(),
/// "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga".to_string(),
/// "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf".to_string(),
/// "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf".to_string(),
/// "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd".to_string(),
/// "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg".to_string(),
/// "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc".to_string()
/// ];
/// let output_values = vec!["fdgacbe cefdb cefbgd gcbe".to_string(),
/// "fcgedb cgb dgebacf gc".to_string(),
/// "cg cg fdcagb cbg".to_string(),
/// "efabcd cedba gadfec cb".to_string(),
/// "gecf egdcabf bgf bfgea".to_string(),
/// "gebdcfa ecba ca fadegcb".to_string(),
/// "cefg dcbef fcge gbcadfe".to_string(),
/// "ed bcgafe cdgba cbgef".to_string(),
/// "gbdfcae bgc cg cgb".to_string(),
/// "fgae cfgab fg bagce".to_string()
/// ];
/// assert_eq!(day08::puzzle_a(&signals, &output_values), 26);
/// ```
pub fn puzzle_a(signals: &Vec<String>, output_values: &Vec<String>) -> usize {
    let display_map = get_default_displays();
    // Three vec levels? Ugh, let me explain.
    // The inner most vec is a 'word', eg, 'cg'
    // the next vec is a 'line', eg 'cg cg fdcagb cbg',
    // the final vec is all lines.
    let _converted_signals: Vec<Vec<Vec<Segment>>> = signals
        .iter()
        .map(|s| convert_line_to_segment_vec(s))
        .collect();
    let converted_outputs: Vec<Vec<Vec<Segment>>> = output_values
        .iter()
        .map(|s| convert_line_to_segment_vec(s))
        .collect();

    let mut lengths_to_look_for: FxHashSet<usize> = FxHashSet::default();
    lengths_to_look_for.insert(display_map.get(&1).unwrap().len());
    lengths_to_look_for.insert(display_map.get(&4).unwrap().len());
    lengths_to_look_for.insert(display_map.get(&7).unwrap().len());
    lengths_to_look_for.insert(display_map.get(&8).unwrap().len());

    let mut count: usize = 0;
    for line in converted_outputs {
        for word in line {
            if lengths_to_look_for.contains(&word.len()) {
                count += 1;
            }
        }
    }

    return count;
}

/// Deduce the wire segments locations, decode the output, and sum
///
/// ```
/// let signals = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb".to_string(),
/// "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec".to_string(),
/// "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef".to_string(),
/// "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega".to_string(),
/// "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga".to_string(),
/// "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf".to_string(),
/// "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf".to_string(),
/// "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd".to_string(),
/// "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg".to_string(),
/// "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc".to_string()
/// ];
/// let output_values = vec!["fdgacbe cefdb cefbgd gcbe".to_string(),
/// "fcgedb cgb dgebacf gc".to_string(),
/// "cg cg fdcagb cbg".to_string(),
/// "efabcd cedba gadfec cb".to_string(),
/// "gecf egdcabf bgf bfgea".to_string(),
/// "gebdcfa ecba ca fadegcb".to_string(),
/// "cefg dcbef fcge gbcadfe".to_string(),
/// "ed bcgafe cdgba cbgef".to_string(),
/// "gbdfcae bgc cg cgb".to_string(),
/// "fgae cfgab fg bagce".to_string()
/// ];
/// assert_eq!(day08::puzzle_b(&signals, &output_values), 61229);
/// ```
pub fn puzzle_b(signals: &Vec<String>, output_values: &Vec<String>) -> usize {
    let display_map = get_default_displays();
    // Three vec levels? Ugh, let me explain.
    // The inner most vec is a 'word', eg, 'cg'
    // the next vec is a 'line', eg 'cg cg fdcagb cbg',
    // the final vec is all lines.
    let converted_signals: Vec<Vec<Vec<Segment>>> = signals
        .iter()
        .map(|s| convert_line_to_segment_vec(s))
        .collect();
    let converted_outputs: Vec<Vec<Vec<Segment>>> = output_values
        .iter()
        .map(|s| convert_line_to_segment_vec(s))
        .collect();

    let mut value = 0;
    for i in 0..converted_signals.len() {
        let signal = &converted_signals[i];
        let output = &converted_outputs[i];
        let dedeuced_map = deduce_map(&signal, &display_map);
        let num = map_output(&output, &dedeuced_map, &display_map);
        value += num;
    }

    return value;
}

fn map_output(
    output: &Vec<Vec<Segment>>,
    deduced_map: &FxHashMap<Segment, Segment>,
    default_map: &FxHashMap<i32, Vec<Segment>>,
) -> usize {
    let mut composite_map: FxHashMap<i32, FxHashSet<Segment>> = FxHashMap::default();

    for (i, values) in default_map {
        let mut mapped_values: FxHashSet<Segment> = FxHashSet::default();
        for x in values {
            mapped_values.insert(*deduced_map.get(x).unwrap());
        }
        composite_map.insert(*i, mapped_values);
    }

    let mut numerals: usize = 0;
    let max_power = output.len();
    for n in 0..output.len() {
        let cur_power = max_power - n - 1;
        let numeral: &Vec<Segment> = output.iter().nth(n).unwrap();
        for (i, values) in composite_map.iter() {
            if numeral.len() == values.len() && numeral.iter().all(|x| values.contains(x)) {
                let positional_num: i32 = i * 10_i32.pow(cur_power.try_into().unwrap());
                let as_usize: usize = positional_num.try_into().unwrap();
                numerals += as_usize;
                break;
            }
        }
    }
    return numerals;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_from_str() {
        assert_eq!("a".parse::<Segment>().unwrap(), Segment::A);
        assert_eq!("b".parse::<Segment>().unwrap(), Segment::B);
        assert_eq!("c".parse::<Segment>().unwrap(), Segment::C);
        assert_eq!("d".parse::<Segment>().unwrap(), Segment::D);
        assert_eq!("e".parse::<Segment>().unwrap(), Segment::E);
        assert_eq!("f".parse::<Segment>().unwrap(), Segment::F);
        assert_eq!("g".parse::<Segment>().unwrap(), Segment::G);
    }

    #[test]
    fn test_convert_str_to_segment_vec() {
        assert_eq!(
            convert_str_to_segment_vec("be"),
            vec![Segment::B, Segment::E]
        );
        assert_eq!(
            convert_str_to_segment_vec("gaf"),
            vec![Segment::G, Segment::A, Segment::F]
        );
    }

    #[test]
    fn test_convert_line_to_segment_vec() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb";
        let expected = vec![
            convert_str_to_segment_vec("be"),
            convert_str_to_segment_vec("cfbegad"),
            convert_str_to_segment_vec("cbdgef"),
            convert_str_to_segment_vec("fgaecd"),
            convert_str_to_segment_vec("cgeb"),
            convert_str_to_segment_vec("fdcge"),
            convert_str_to_segment_vec("agebfd"),
            convert_str_to_segment_vec("fecdb"),
            convert_str_to_segment_vec("fabcd"),
            convert_str_to_segment_vec("edb"),
        ];
        assert_eq!(convert_line_to_segment_vec(input), expected);
    }

    #[test]
    fn test_deduce_map() {
        let input = convert_line_to_segment_vec(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab",
        );
        let defaults = get_default_displays();
        let result = deduce_map(&input, &defaults);
        let mut expected = FxHashMap::default();
        /*
         aaaa          dddd
        b    c        e    a
        b    c        e    a
         dddd    ->    ffff
        e    f        g    b
        e    f        g    b
         gggg          cccc
        */
        expected.insert(Segment::A, Segment::D); // What was originally A is now a D
        expected.insert(Segment::B, Segment::E); // What was originally B is now a E
        expected.insert(Segment::C, Segment::A); // What was originally C is now a A
        expected.insert(Segment::D, Segment::F);
        expected.insert(Segment::E, Segment::G);
        expected.insert(Segment::F, Segment::B);
        expected.insert(Segment::G, Segment::C);
        assert_eq!(result.get(&Segment::A), expected.get(&Segment::A));
        assert_eq!(result.get(&Segment::B), expected.get(&Segment::B));
        assert_eq!(result.get(&Segment::C), expected.get(&Segment::C));
        assert_eq!(result.get(&Segment::D), expected.get(&Segment::D));
        assert_eq!(result.get(&Segment::E), expected.get(&Segment::E));
        assert_eq!(result.get(&Segment::F), expected.get(&Segment::F));
        assert_eq!(result.get(&Segment::G), expected.get(&Segment::G));
    }

    #[test]
    fn test_map_output() {
        let input = convert_line_to_segment_vec(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab",
        );
        let defaults = get_default_displays();
        let deduced_map = deduce_map(&input, &defaults);

        let output = convert_line_to_segment_vec("cdfeb fcadb cdfeb cdbaf");

        assert_eq!(map_output(&output, &deduced_map, &defaults), 5353)
    }
}
