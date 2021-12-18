pub use filelib::load;

#[derive(Debug, Clone, PartialEq)]
pub enum SnailNumber {
    Num(u8),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

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

fn split(snail: &mut Box<SnailNumber>) -> bool {
    match snail.as_mut() {
        SnailNumber::Num(n) => {
            if *n >= 10 {
                **snail = SnailNumber::Pair(
                    Box::new(SnailNumber::Num(*n / 2)),
                    Box::new(SnailNumber::Num((*n + 1) / 2)),
                );
                true
            } else {
                false
            }
        }
        SnailNumber::Pair(l, r) => {
            let mut ok = split(l);
            if !ok {
                ok = split(r);
            }

            ok
        }
    }
}

fn add_to_right(snail: &mut Box<SnailNumber>, val: u8) {
    match snail.as_mut() {
        SnailNumber::Num(n) => *n += val,
        SnailNumber::Pair(_, r) => add_to_right(r, val),
    }
}

fn add_to_left(snail: &mut Box<SnailNumber>, val: u8) {
    match snail.as_mut() {
        SnailNumber::Num(n) => *n += val,
        SnailNumber::Pair(l, _) => add_to_left(l, val),
    }
}

fn explode(snail: &mut Box<SnailNumber>, depth: u8) -> (bool, Option<u8>, Option<u8>) {
    match snail.as_mut() {
        SnailNumber::Num(_) => (false, None, None),
        SnailNumber::Pair(l, r) => {
            if depth == 4 {
                match (&**l, &**r) {
                    (SnailNumber::Num(left), SnailNumber::Num(right)) => {
                        let ret = (true, Some(*left), Some(*right));
                        **snail = SnailNumber::Num(0);
                        ret
                    }
                    (_, _) => panic!("Tree too deep already!"),
                }
            } else {
                let left = explode(l, depth + 1);
                if left.0 {
                    if let Some(val) = left.2 {
                        add_to_left(r, val);
                        (true, left.1, None)
                    } else {
                        left
                    }
                } else {
                    let right = explode(r, depth + 1);
                    if right.0 {
                        if let Some(val) = right.1 {
                            add_to_right(l, val);
                            (true, None, right.2)
                        } else {
                            right
                        }
                    } else {
                        (false, None, None)
                    }
                }
            }
        }
    }
}

fn calc_magnitude(snail: &SnailNumber) -> u64 {
    match snail {
        SnailNumber::Num(n) => *n as u64,
        SnailNumber::Pair(l, r) => 3 * calc_magnitude(l) + 2 * calc_magnitude(r),
    }
}

fn reduce(snail: &mut Box<SnailNumber>) {
    loop {
        let x = explode(snail, 0);
        if !x.0 {
            let x = split(snail);
            if !x {
                break;
            }
        }
    }
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
        reduce(&mut sum);
    }
    calc_magnitude(&sum)
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
            reduce(&mut x_plus_y);
            reduce(&mut y_plus_x);
            max = u64::max(
                max,
                u64::max(calc_magnitude(&x_plus_y), calc_magnitude(&y_plus_x)),
            );
        }
    }
    return max;
}

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
