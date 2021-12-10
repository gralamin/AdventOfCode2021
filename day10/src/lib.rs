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
    fn get_points(&self) -> Option<u64> {
        match self {
            Chunks::CloseRound => Some(3),
            Chunks::CloseSquare => Some(57),
            Chunks::CloseCurly => Some(1197),
            Chunks::CloseTriangle => Some(25137),
            Chunks::OpenRound => Some(1),
            Chunks::OpenSquare => Some(2),
            Chunks::OpenCurly => Some(3),
            Chunks::OpenTriangle => Some(4),
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

fn line_is_corrupted(line: &str) -> (bool, u64) {
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

fn complete_line(line: &str) -> u64 {
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
            // This isn't corrupt, so I can just pop it, its fine.
            unclosed_sets.pop();
        }
    }

    let mut score: u64 = 0;
    while let Some(stack_value) = unclosed_sets.pop() {
        score *= 5;
        score += stack_value.get_points().unwrap();
    }

    return score;
}

/// Check if each line is corrupted, and add together thep oints of the ones that are corrupted
///
/// ```
/// let lines = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]";
/// assert_eq!(day10::puzzle_a(lines), 26397);
/// ```
pub fn puzzle_a(lines: &str) -> u64 {
    let mut sum = 0;
    for line in lines.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let ret_tuple = line_is_corrupted(line);
        if ret_tuple.0 {
            sum += ret_tuple.1;
        }
    }
    return sum;
}

/// Check if each line is corrupted, complete the uncompleted one, and give the middle score
///
/// ```
/// let lines = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]";
/// assert_eq!(day10::puzzle_b(lines), 288957);
/// ```
pub fn puzzle_b(lines: &str) -> u64 {
    let incomplete_lines = lines
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter(|x| !line_is_corrupted(x).0);
    let mut results: Vec<u64> = incomplete_lines.map(|x| complete_line(x)).collect();
    results.sort();
    let middle = results.len() / 2;
    return *results.iter().nth(middle).unwrap();
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

    #[test]
    fn test_complete_line() {
        assert_eq!(complete_line("[({(<(())[]>[[{[]{<()<>>"), 288957);
        assert_eq!(complete_line("[(()[<>])]({[<{<<[]>>("), 5566);
        assert_eq!(complete_line("(((({<>}<{<{<>}{[]{[]{}"), 1480781);
        assert_eq!(complete_line("<[[]]>}<{[{[{[]{()[[[]"), 995444);
        assert_eq!(complete_line("<{([{{}}[<[[[<>{}]]]>[]]"), 294);
    }
}
