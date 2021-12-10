pub use filelib::load;
use std::collections::HashSet;
// chunk pairs: (), [], {}, <>

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Chunks {
    OpenRound,
    CloseRound,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    OpenTriangle,
    CloseTriangle,
}

impl Chunks {
    fn get_points(&self) -> Option<u32> {
        match self {
            Chunks::CloseRound => Some(3),
            Chunks::CloseSquare => Some(57),
            Chunks::CloseCurly => Some(1197),
            Chunks::CloseTriangle => Some(25137),
            _ => None,
        }
    }

    fn get_matching_end_character(&self) -> Option<Chunks> {
        match self {
            Chunks::OpenRound => Some(Chunks::CloseRound),
            Chunks::OpenSquare => Some(Chunks::CloseSquare),
            Chunks::OpenCurly => Some(Chunks::CloseCurly),
            Chunks::OpenTriangle => Some(Chunks::CloseTriangle),
            _ => None,
        }
    }
}

impl std::str::FromStr for Chunks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Chunks::OpenRound),
            ")" => Ok(Chunks::CloseRound),
            "{" => Ok(Chunks::OpenCurly),
            "}" => Ok(Chunks::CloseCurly),
            "[" => Ok(Chunks::OpenSquare),
            "]" => Ok(Chunks::CloseSquare),
            "<" => Ok(Chunks::OpenTriangle),
            ">" => Ok(Chunks::CloseTriangle),
            _ => Err(()),
        }
    }
}

fn line_is_corrupted(line: &str) -> (bool, u32) {
    let mut unclosed_sets: Vec<Chunks> = Vec::new();
    let mut open_chunk_set: HashSet<Chunks> = HashSet::new();
    open_chunk_set.insert(Chunks::OpenRound);
    open_chunk_set.insert(Chunks::OpenSquare);
    open_chunk_set.insert(Chunks::OpenCurly);
    open_chunk_set.insert(Chunks::OpenTriangle);

    for x in line.chars() {
        let cur_chunk: Chunks = x.to_string().parse::<Chunks>().unwrap();
        if open_chunk_set.contains(&cur_chunk) {
            unclosed_sets.push(cur_chunk);
        } else {
            if let Some(expected_opening) = unclosed_sets.pop() {
                if cur_chunk != expected_opening.get_matching_end_character().unwrap() {
                    return (true, cur_chunk.get_points().unwrap());
                }
            }
        }
    }
    return (false, 0);
}

/// Check if each line is corrupted, and add together thep oints of the ones that are corrupted
///
/// ```
/// let lines = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]";
/// assert_eq!(day10::puzzle_a(lines), 26397);
/// ```
pub fn puzzle_a(lines: &str) -> u32 {
    let mut sum = 0;
    for line in lines.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let ret_tuple = line_is_corrupted(line);
        if ret_tuple.0 {
            sum += ret_tuple.1;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_line_corrupted() {
        assert_eq!(line_is_corrupted("{([(<{}[<>[]}>{[]{[(<()>"), (true, 1197));
        assert_eq!(line_is_corrupted("[[<[([]))<([[{}[[()]]]"), (true, 3));
        assert_eq!(line_is_corrupted("[{[{({}]{}}([{[{{{}}([]"), (true, 57));
        assert_eq!(line_is_corrupted("[<(<(<(<{}))><([]([]()"), (true, 3));
        assert_eq!(line_is_corrupted("<{([([[(<>()){}]>(<<{{"), (true, 25137));
        assert_eq!(line_is_corrupted("<{([{{}}[<[[[<>{}]]]>[]]"), (false, 0));
    }
}
