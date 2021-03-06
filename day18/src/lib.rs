pub use filelib::load;

#[derive(Debug, Clone, PartialEq)]
pub enum SnailNumber {
    Num(u8),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

impl SnailNumber {
    fn magnitude(&self) -> u64 {
        return match self {
            // On a number, return its value
            SnailNumber::Num(n) => *n as u64,
            // On a pair, return 3 * l + 2 * r
            SnailNumber::Pair(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        };
    }

    // Return false if we don't split.
    fn split(&mut self) -> bool {
        match self {
            // If we are a number, split on 10 or higher, into a pair (by dividing by 2)
            // pair left should be rounded down, right should be rounded up
            SnailNumber::Num(n) => {
                if *n >= 10 {
                    *self = SnailNumber::Pair(
                        Box::new(SnailNumber::Num(*n / 2)),
                        Box::new(SnailNumber::Num((*n + 1) / 2)),
                    );
                    return true;
                } else {
                    return false;
                }
            }
            // If we are a pair, split left. If we fail to split on left, split on right.
            SnailNumber::Pair(l, r) => {
                let mut ok = l.split();
                if !ok {
                    ok = r.split();
                }

                return ok;
            }
        }
    }

    fn explode(&mut self, depth: u8) -> (bool, Option<u8>, Option<u8>) {
        return match self {
            SnailNumber::Num(_) => (false, None, None),
            SnailNumber::Pair(l, r) => {
                if depth == 4 {
                    // We need to get values out of boxes...
                    match (&**l, &**r) {
                        (SnailNumber::Num(left), SnailNumber::Num(right)) => {
                            // Set self to 0, and return up the values to be exploded.
                            let ret = (true, Some(*left), Some(*right));
                            *self = SnailNumber::Num(0);
                            return ret;
                        }
                        (_, _) => panic!("Tree too deep already!"),
                    }
                } else {
                    // Try exploding left
                    let left = l.explode(depth + 1);
                    if left.0 {
                        // If it does explode, grab the right value, and add it to our left value.
                        if let Some(val) = left.2 {
                            r.add_to_left(val);
                            // Indicate up we have a value exploded, and still need to consume the left value.
                            return (true, left.1, None);
                        } else {
                            // Right already consumed, leave left alone.
                            return left;
                        }
                    } else {
                        // Try exploding right
                        let right = r.explode(depth + 1);
                        if right.0 {
                            // if it does explode, grab the left value, and add it to our right value.
                            if let Some(val) = right.1 {
                                l.add_to_right(val);
                                // indicate we have a value exploded, and still need to consume the right value.
                                return (true, None, right.2);
                            } else {
                                // left already consumed, leave alone.
                                return right;
                            }
                        } else {
                            // No explode.
                            return (false, None, None);
                        }
                    }
                }
            }
        };
    }

    fn reduce(&mut self) {
        // Try exploding, if I can't, split. Repeat until neither can happen.
        loop {
            let x = self.explode(0);
            if !x.0 {
                let x = self.split();
                if !x {
                    break;
                }
            }
        }
    }

    fn add_to_right(&mut self, val: u8) {
        // In the right most number, add val
        match self {
            SnailNumber::Num(n) => *n += val,
            SnailNumber::Pair(_, r) => r.add_to_right(val),
        }
    }

    fn add_to_left(&mut self, val: u8) {
        // In the left most number, add val
        match self {
            SnailNumber::Num(n) => *n += val,
            SnailNumber::Pair(l, _) => l.add_to_left(val),
        }
    }
}

// Parse read snail number character by character, recursively.
fn read_snail_num(text: &str) -> (Box<SnailNumber>, usize) {
    if text[0..1] == *"[" {
        let (left, left_id) = read_snail_num(&text[1..]);
        assert_eq!(text[left_id + 1..left_id + 2], *",");
        let (right, right_id) = read_snail_num(&text[(left_id + 2)..]);
        assert_eq!(text[left_id + right_id + 2..left_id + right_id + 3], *"]");
        (
            Box::new(SnailNumber::Pair(left, right)),
            left_id + right_id + 3,
        )
    } else {
        // Must be a number.
        (
            Box::new(SnailNumber::Num(text[0..1].parse::<u8>().unwrap())),
            1,
        )
    }
}

/// Parse the homework input
///
/// ```
/// let v = "[1,2]\n[[1,2],3]";
/// let expected = vec![
///     Box::new(day18::SnailNumber::Pair(
///         Box::new(day18::SnailNumber::Num(1)),
///         Box::new(day18::SnailNumber::Num(2))
///     )),
///     Box::new(day18::SnailNumber::Pair(
///         Box::new(day18::SnailNumber::Pair(
///             Box::new(day18::SnailNumber::Num(1)),
///             Box::new(day18::SnailNumber::Num(2))
///         )),
///         Box::new(day18::SnailNumber::Num(3))
///     ))
/// ];
/// assert_eq!(day18::parse(v), expected);
/// ```
pub fn parse(input: &str) -> Vec<Box<SnailNumber>> {
    input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| read_snail_num(x).0)
        .collect()
}

/// Parse the homework input
///
/// ```
/// let v = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
/// let parsed = day18::parse(v);
/// assert_eq!(day18::puzzle_a(&parsed), 4140);
/// ```
pub fn puzzle_a(v: &[Box<SnailNumber>]) -> u64 {
    let mut sum = v[0].clone();
    for snail in v[1..].iter() {
        sum = Box::new(SnailNumber::Pair(sum, snail.clone()));
        sum.reduce();
    }
    sum.magnitude()
}

/// Try adding any two numbers together in any order, and find the max magnitude
///
/// ```
/// let v = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
/// let parsed = day18::parse(v);
/// assert_eq!(day18::puzzle_b(&parsed), 3993);
/// ```
pub fn puzzle_b(v: &[Box<SnailNumber>]) -> u64 {
    let mut max = 0;
    for (i, snail) in v.iter().enumerate() {
        for second_snail in v[i + 1..].iter() {
            let mut x_plus_y = Box::new(SnailNumber::Pair(snail.clone(), second_snail.clone()));
            let mut y_plus_x = Box::new(SnailNumber::Pair(second_snail.clone(), snail.clone()));
            x_plus_y.reduce();
            y_plus_x.reduce();
            max = u64::max(max, u64::max(x_plus_y.magnitude(), y_plus_x.magnitude()));
        }
    }
    return max;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_snail(snail: &SnailNumber) {
        match snail {
            SnailNumber::Num(n) => {
                print!("{}", *n)
            }
            SnailNumber::Pair(l, r) => {
                print!("[");
                print_snail(l);
                print!(",");
                print_snail(r);
                print!("]");
            }
        }
    }

    #[test]
    fn test_magnitude() {
        let parsed = parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        let num = parsed.first().unwrap();
        print_snail(num);
        assert_eq!(num.magnitude(), 4140);
    }

    #[test]
    fn test_split() {
        let mut num = vec![Box::new(SnailNumber::Pair(
            Box::new(SnailNumber::Num(11)),
            Box::new(SnailNumber::Num(3)),
        ))];
        let num: &mut Box<SnailNumber> = num.get_mut(0).unwrap();
        let mut expected = Box::new(SnailNumber::Pair(
            Box::new(SnailNumber::Pair(
                Box::new(SnailNumber::Num(5)),
                Box::new(SnailNumber::Num(6)),
            )),
            Box::new(SnailNumber::Num(3)),
        ));

        assert_eq!(num.split(), true);
        assert_eq!(num, &mut expected);
    }

    #[test]
    fn test_explode() {
        let mut num = Box::new(SnailNumber::Pair(
            Box::new(SnailNumber::Pair(
                Box::new(SnailNumber::Pair(
                    Box::new(SnailNumber::Pair(
                        Box::new(SnailNumber::Pair(
                            Box::new(SnailNumber::Num(9)),
                            Box::new(SnailNumber::Num(8)),
                        )),
                        Box::new(SnailNumber::Num(1)),
                    )),
                    Box::new(SnailNumber::Num(2)),
                )),
                Box::new(SnailNumber::Num(3)),
            )),
            Box::new(SnailNumber::Num(4)),
        ));

        let expected = Box::new(SnailNumber::Pair(
            Box::new(SnailNumber::Pair(
                Box::new(SnailNumber::Pair(
                    Box::new(SnailNumber::Pair(
                        Box::new(SnailNumber::Num(0)),
                        Box::new(SnailNumber::Num(9)),
                    )),
                    Box::new(SnailNumber::Num(2)),
                )),
                Box::new(SnailNumber::Num(3)),
            )),
            Box::new(SnailNumber::Num(4)),
        ));

        let (result, _, _) = num.explode(0);

        assert_eq!(result, true);
        assert_eq!(num, expected);
    }

    #[test]
    fn test_reduce() {
        let mut parsed = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let num = parsed.get_mut(0).unwrap();

        let parsed = parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        let expected = parsed.get(0).unwrap();

        num.reduce();
        assert_eq!(num, expected);
    }

    #[test]
    fn test_add_left() {
        let mut parsed = parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let base = parsed.get_mut(0).unwrap();

        let adder = 4;

        let mut parsed = parse("[[[[8,3],4],4],[7,[[8,4],9]]]");
        let expected = parsed.get_mut(0).unwrap();

        base.add_to_left(adder);
        assert_eq!(base, expected);
    }

    #[test]
    fn test_add_right() {
        let mut parsed = parse("[[[[4,3],4],4],[7,[[8,4],1]]]");
        let base = parsed.get_mut(0).unwrap();

        let adder = 4;
        let mut parsed = parse("[[[[4,3],4],4],[7,[[8,4],5]]]");
        let expected = parsed.get_mut(0).unwrap();

        base.add_to_right(adder);
        assert_eq!(base, expected);
    }
}
