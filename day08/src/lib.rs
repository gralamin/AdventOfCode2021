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

#[derive(Debug, PartialEq)]
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
}

/*
let input = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
    "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
    "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
    "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
    "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
    "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
    "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
    "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
    "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
];
let expected_signals = vec!["be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb",
"edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec",
"fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef",
"fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega",
"aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga",
"fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf",
"dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf",
"bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd",
"egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg",
"gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc"
]
let expected_output_values  vec!["fdgacbe cefdb cefbgd gcbe",
"fcgedb cgb dgebacf gc",
"cg cg fdcagb cbg",
"efabcd cedba gadfec cb",
"gecf egdcabf bgf bfgea",
"gebdcfa ecba ca fadegcb",
"cefg dcbef fcge gbcadfe",
"ed bcgafe cdgba cbgef",
"gbdfcae bgc cg cgb",
"fgae cfgab fg bagce"
];
*/
