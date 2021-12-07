extern crate filelib;

pub use filelib::load;
pub use filelib::parse_csv_i32_lines;

fn get_fuel_cost_to_align(initial_pos: &Vec<i32>, final_pos: i32) -> i32 {
    return initial_pos.iter().map(|s| (final_pos - s).abs()).sum();
}

// We can use a binary search to find the lowest value, as this should form a parabola if you graph them
// essentially one point will be the lowest with each side of it being higher.
// As we are going to have to check two sides, we will use recursion to track these seperately.
fn binary_search_fuel(crab_pos: &Vec<i32>, lower_bound: i32, upper_bound: i32) -> (i32, i32) {
    // If we ever cross, I've screwed up
    if lower_bound > upper_bound {
        return (i32::MAX, i32::MAX);
    }
    if lower_bound == upper_bound {
        return (lower_bound, get_fuel_cost_to_align(crab_pos, lower_bound));
    }

    let lowest_pos = (lower_bound + upper_bound) / 2;
    let lowest_cost = get_fuel_cost_to_align(crab_pos, lowest_pos);

    // if we are exactly one different, we can end up in an infinite loop.
    let left_upper_bound = if (lower_bound - lowest_pos).abs() > 1 {lowest_pos} else {lower_bound};
    let right_lower_bound = if {upper_bound - lowest_pos}.abs() > 1 {lowest_pos} else {upper_bound};

    let left_tree_tuple = binary_search_fuel(crab_pos, lower_bound, left_upper_bound);
    if left_tree_tuple.1 < lowest_cost {
        // If its decreasing to the left, then to the right it must be increasing.
        return left_tree_tuple;
    }
    let right_tree_tuple = binary_search_fuel(crab_pos, right_lower_bound, upper_bound);
    if right_tree_tuple.1 < lowest_cost {
        // If its decreasing to the right, then the left must be increasing.
        return right_tree_tuple;
    }
    // We got it right first try
    return (lowest_pos, lowest_cost);
}

/// Get the least cost position to move to, but only return the cost.
///
/// ```
/// let crab_pos = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
/// assert_eq!(day07::puzzle_a(&crab_pos), 37);
/// ```
pub fn puzzle_a(crab_pos: &Vec<i32>) -> i32 {
    let lowest_value = crab_pos.iter().min().unwrap();
    let highest_value = crab_pos.iter().max().unwrap();
    let tuple = binary_search_fuel(crab_pos, *lowest_value, *highest_value);
    let _position = tuple.0;
    let cost = tuple.1;
    return cost;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fuel_cost_to_align() {
        let initial = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(get_fuel_cost_to_align(&initial, 2), 37);
        assert_eq!(get_fuel_cost_to_align(&initial, 1), 41);
        assert_eq!(get_fuel_cost_to_align(&initial, 3), 39);
        assert_eq!(get_fuel_cost_to_align(&initial, 10), 71);
    }

    #[test]
    fn test_binary_search_fuel() {
        let crab_pos = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let lowest_value = crab_pos.iter().min().unwrap();
        let highest_value = crab_pos.iter().max().unwrap();
        let returned = binary_search_fuel(&crab_pos, *lowest_value, *highest_value);
        assert_eq!(returned.0, 2);
        assert_eq!(returned.1, 37);
    }
}
