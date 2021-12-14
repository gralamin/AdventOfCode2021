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

#[derive(Debug)]
struct Polymer {
    data: String,
}

impl Polymer {
    fn new(data: String) -> Polymer {
        return Polymer { data: data };
    }

    fn get_pair_iter(&self) -> PairIter {
        return PairIter {
            base_data: self,
            cur_index: 0,
        };
    }
}

#[derive(Debug)]
struct PairIter<'a> {
    base_data: &'a Polymer,
    cur_index: usize,
}

impl<'a> Iterator for PairIter<'a> {
    type Item = PolyPair;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_index + 1 >= self.base_data.data.len() {
            return None;
        }
        let a = &self.base_data.data[self.cur_index..self.cur_index + 1];
        let b = &self.base_data.data[self.cur_index + 1..self.cur_index + 2];
        self.cur_index += 1;
        return Some(Self::Item::from(a, b));
    }
}

fn polymer_cycle(polymer: &Polymer, rules: &FxHashMap<PolyPair, String>) -> Polymer {
    let mut new_data: Vec<String> = Vec::new();

    let mut first = true;
    for pair in polymer.get_pair_iter() {
        if let Some(x) = rules.get(&pair) {
            if first {
                new_data.push(pair.a);
            }
            new_data.push(x.to_string());
            new_data.push(pair.b);
        } else {
            if first {
                new_data.push(pair.a);
            }
            new_data.push(pair.b);
        }
        first = false;
    }

    return Polymer::new(new_data.join(""));
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

/// Get the complete set of characters that appears
///
/// ```
/// use rustc_hash::{FxHashMap, FxHashSet};
/// use day14::PolyPair;
/// let mut rules: FxHashMap<PolyPair, String> = FxHashMap::default();
/// rules.insert(PolyPair::new("CH"), "B".to_string());
/// rules.insert(PolyPair::new("HH"), "N".to_string());
/// let template = "NNCB";
/// let mut alphabet = FxHashSet::default();
/// alphabet.insert("N".to_string());
/// alphabet.insert("C".to_string());
/// alphabet.insert("B".to_string());
/// alphabet.insert("H".to_string());
/// assert_eq!(day14::get_alphabet(template, &rules), alphabet)
/// ```
pub fn get_alphabet(
    polymer_template: &str,
    rules: &FxHashMap<PolyPair, String>,
) -> FxHashSet<String> {
    let mut alphabet = FxHashSet::default();
    for c in polymer_template.chars() {
        alphabet.insert(c.to_string());
    }
    for (rule, value) in rules {
        alphabet.insert(rule.a.clone());
        alphabet.insert(rule.b.clone());
        alphabet.insert(value.to_string());
    }
    return alphabet;
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
/// let alphabet = day14::get_alphabet(template, &rules);
/// assert_eq!(day14::puzzle_a(template, &rules, &alphabet), 1588);
/// ```
pub fn puzzle_a(
    polymer_template: &str,
    rules: &FxHashMap<PolyPair, String>,
    alphabet: &FxHashSet<String>,
) -> usize {
    // First of all apply the template 10 times.
    let mut polymer = Polymer::new(polymer_template.to_string());
    for _ in 0..10 {
        polymer = polymer_cycle(&polymer, &rules);
    }

    let mut top_count = 0;
    let mut bottom_count = polymer.data.len();
    for character in alphabet {
        let count = polymer.data.matches(character).count();
        if count > top_count {
            top_count = count;
        }
        if count < bottom_count {
            bottom_count = count;
        }
    }
    return top_count - bottom_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_polymer() -> Polymer {
        let input = "NNCB".to_string();
        return Polymer::new(input);
    }

    fn make_rules() -> FxHashMap<PolyPair, String> {
        let mut map = FxHashMap::default();
        map.insert(PolyPair::new("CH"), "B".to_string());
        map.insert(PolyPair::new("HH"), "N".to_string());
        map.insert(PolyPair::new("CB"), "H".to_string());
        map.insert(PolyPair::new("NH"), "C".to_string());
        map.insert(PolyPair::new("HB"), "C".to_string());
        map.insert(PolyPair::new("HC"), "B".to_string());
        map.insert(PolyPair::new("HN"), "C".to_string());
        map.insert(PolyPair::new("NN"), "C".to_string());
        map.insert(PolyPair::new("BH"), "H".to_string());
        map.insert(PolyPair::new("NC"), "B".to_string());
        map.insert(PolyPair::new("NB"), "B".to_string());
        map.insert(PolyPair::new("BN"), "B".to_string());
        map.insert(PolyPair::new("BB"), "N".to_string());
        map.insert(PolyPair::new("BC"), "B".to_string());
        map.insert(PolyPair::new("CC"), "N".to_string());
        map.insert(PolyPair::new("CN"), "C".to_string());
        return map;
    }

    #[test]
    fn test_polymer_get_pairs() {
        let polymer = make_polymer();
        let result: Vec<PolyPair> = polymer.get_pair_iter().collect();
        assert_eq!(result.len(), 3);
        assert_eq!(
            result,
            vec![
                PolyPair::new("NN"),
                PolyPair::new("NC"),
                PolyPair::new("CB")
            ]
        );
    }

    #[test]
    fn test_polymer_cycle() {
        let rules = make_rules();
        let polymer = make_polymer();
        let result = polymer_cycle(&polymer, &rules);
        assert_eq!(result.data, "NCNBCHB");
        let result_2 = polymer_cycle(&result, &rules);
        assert_eq!(result_2.data, "NBCCNBBBCBHCB");
        let result_3 = polymer_cycle(&result_2, &rules);
        assert_eq!(result_3.data, "NBBBCNCCNBBNBNBBCHBHHBCHB");
        let result_4 = polymer_cycle(&result_3, &rules);
        assert_eq!(
            result_4.data,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
    }
}
