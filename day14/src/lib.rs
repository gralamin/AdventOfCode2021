pub use filelib::{load, split_lines_by_blanks};
pub use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PolyPair {
    pub a: String,
    pub b: String,
}

impl PolyPair {
    pub fn new(pair: &str) -> PolyPair {
        return PolyPair {
            a: pair[..1].to_string(),
            b: pair[1..2].to_string(),
        };
    }

    pub fn from(a: &str, b: &str) -> PolyPair {
        return PolyPair {
            a: a.to_string(),
            b: b.to_string(),
        };
    }
}

/// Parse a set of rules
///
/// ```
/// use rustc_hash::FxHashMap;
/// use day14::PolyPair;
/// let mut expected: FxHashMap<PolyPair, String> = FxHashMap::default();
/// expected.insert(PolyPair::new("CH"), "B".to_string());
/// expected.insert(PolyPair::new("HH"), "N".to_string());
/// let input = vec!["CH -> B".to_string(), "HH -> N".to_string(), "".to_string()];
/// assert_eq!(day14::create_rules(input), expected);
/// ```
pub fn create_rules(inputs: Vec<String>) -> FxHashMap<PolyPair, String> {
    let mut map: FxHashMap<PolyPair, String> = FxHashMap::default();

    for x in inputs.iter().map(|x| x.trim()).filter(|x| !x.is_empty()) {
        let (pair, value) = x.split_once("->").unwrap();
        map.insert(PolyPair::new(pair.trim()), value.trim().to_string());
    }

    return map;
}

/// Cycle the polymer 10 times, then get the top two character counts, and subtract from each other
///
/// ```
/// use rustc_hash::{FxHashMap, FxHashSet};
/// use day14::PolyPair;
/// let mut rules: FxHashMap<PolyPair, String> = FxHashMap::default();
/// rules.insert(PolyPair::new("CH"), "B".to_string());
/// rules.insert(PolyPair::new("HH"), "N".to_string());
/// rules.insert(PolyPair::new("CB"), "H".to_string());
/// rules.insert(PolyPair::new("NH"), "C".to_string());
/// rules.insert(PolyPair::new("HB"), "C".to_string());
/// rules.insert(PolyPair::new("HC"), "B".to_string());
/// rules.insert(PolyPair::new("HN"), "C".to_string());
/// rules.insert(PolyPair::new("NN"), "C".to_string());
/// rules.insert(PolyPair::new("BH"), "H".to_string());
/// rules.insert(PolyPair::new("NC"), "B".to_string());
/// rules.insert(PolyPair::new("NB"), "B".to_string());
/// rules.insert(PolyPair::new("BN"), "B".to_string());
/// rules.insert(PolyPair::new("BB"), "N".to_string());
/// rules.insert(PolyPair::new("BC"), "B".to_string());
/// rules.insert(PolyPair::new("CC"), "N".to_string());
/// rules.insert(PolyPair::new("CN"), "C".to_string());
/// let template = "NNCB";
/// assert_eq!(day14::puzzle_a(template, &rules), 1588);
/// ```
pub fn puzzle_a(polymer_template: &str, rules: &FxHashMap<PolyPair, String>) -> usize {
    return common_puzzle(polymer_template, rules, 10);
}

/// Takes too long to actually do it, so use counting instead.
fn common_puzzle(
    polymer_template: &str,
    rules: &FxHashMap<PolyPair, String>,
    num_runs: usize,
) -> usize {
    let mut counts = FxHashMap::default();
    for i in polymer_template.chars().collect::<Vec<_>>().windows(2) {
        // Create pairs for everything in the template at 1.
        let pair = PolyPair::from(&i[0].to_string(), &i[1].to_string());
        *counts.entry(pair).or_default() += 1;
    }

    for _ in 0..num_runs {
        let mut current = FxHashMap::default();
        for (key, c) in counts.iter() {
            // Have a matching rule, so increase counts of results of inserting.
            if let Some(letter) = rules.get(key) {
                let new_pair_left = PolyPair::from(&key.a, &letter);
                let new_pair_right = PolyPair::from(&letter, &key.b);
                *current.entry(new_pair_left).or_default() += c;
                *current.entry(new_pair_right).or_default() += c;
            }
        }
        counts = current;
    }

    // We have all the counts, extract the max and min. Turns out, fold can help us do this.
    // We basically count all the occurances of the second characters we have.
    let mut final_counts = counts.iter().fold(
        FxHashMap::<String, usize>::default(),
        |mut m: FxHashMap<String, usize>, (k, v): (&PolyPair, &usize)| {
            *m.entry(k.b.clone()).or_default() += v;
            return m;
        },
    );
    // Only thing that will be missing is first character.
    let first_char: String = polymer_template.chars().nth(0).unwrap().to_string();
    *final_counts.entry(first_char).or_default() += 1;
    // Magicy methods to get max and min easy :O
    let max = final_counts.iter().max_by_key(|(_, i)| *i).unwrap().1;
    let min = final_counts.iter().min_by_key(|(_, i)| *i).unwrap().1;
    return max - min;
}

/// Cycle the polymer 10 times, then get the top two character counts, and subtract from each other
///
/// ```
/// use rustc_hash::{FxHashMap, FxHashSet};
/// use day14::PolyPair;
/// let mut rules: FxHashMap<PolyPair, String> = FxHashMap::default();
/// rules.insert(PolyPair::new("CH"), "B".to_string());
/// rules.insert(PolyPair::new("HH"), "N".to_string());
/// rules.insert(PolyPair::new("CB"), "H".to_string());
/// rules.insert(PolyPair::new("NH"), "C".to_string());
/// rules.insert(PolyPair::new("HB"), "C".to_string());
/// rules.insert(PolyPair::new("HC"), "B".to_string());
/// rules.insert(PolyPair::new("HN"), "C".to_string());
/// rules.insert(PolyPair::new("NN"), "C".to_string());
/// rules.insert(PolyPair::new("BH"), "H".to_string());
/// rules.insert(PolyPair::new("NC"), "B".to_string());
/// rules.insert(PolyPair::new("NB"), "B".to_string());
/// rules.insert(PolyPair::new("BN"), "B".to_string());
/// rules.insert(PolyPair::new("BB"), "N".to_string());
/// rules.insert(PolyPair::new("BC"), "B".to_string());
/// rules.insert(PolyPair::new("CC"), "N".to_string());
/// rules.insert(PolyPair::new("CN"), "C".to_string());
/// let template = "NNCB";
/// assert_eq!(day14::puzzle_b(template, &rules), 2188189693529);
/// ```
pub fn puzzle_b(polymer_template: &str, rules: &FxHashMap<PolyPair, String>) -> usize {
    return common_puzzle(polymer_template, rules, 40);
}
