use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum SubDirection {
    Forward,
    Down,
    Up,
}

impl FromStr for SubDirection {
    type Err = ();

    fn from_str(input: &str) -> Result<SubDirection, Self::Err> {
        match input {
            "forward" => Ok(SubDirection::Forward),
            "up" => Ok(SubDirection::Up),
            "down" => Ok(SubDirection::Down),
            _ => Err(()),
        }
    }
}

/// Converts lines from the input file to sub directions
///
/// ```
/// let values = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"];
/// let expected = vec![(day02::SubDirection::Forward, 5),
///                     (day02::SubDirection::Down, 5),
///                     (day02::SubDirection::Forward, 8),
///                     (day02::SubDirection::Up, 3),
///                     (day02::SubDirection::Down, 8),
///                     (day02::SubDirection::Forward, 2)];
/// assert_eq!(day02::directional_commands_from_strs(values), expected);
/// ```
pub fn directional_commands_from_strs(str_list: Vec<&str>) -> Vec<(SubDirection, i32)> {
    let mut result: Vec<(SubDirection, i32)> = Vec::new();
    for cur_str in str_list {
        let mut split_iter = cur_str.split(" ");
        let dir = SubDirection::from_str(split_iter.next().unwrap()).unwrap();
        let value = split_iter.next().unwrap().parse::<i32>().unwrap();
        result.push((dir, value));
    }
    return result;
}
