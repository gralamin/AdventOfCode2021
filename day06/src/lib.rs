extern crate filelib;

pub use filelib::load;
pub use filelib::parse_csv_i32_lines;

#[derive(Debug)]
struct LanternFish {
    timer: i32,
}

trait ExpoentialFish {
    fn cycle(&mut self) -> Option<LanternFish>;
}

impl ExpoentialFish for LanternFish {
    fn cycle(&mut self) -> Option<LanternFish> {
        self.timer -= 1;
        if self.timer < 0 {
            self.timer = 6;
            return Some(LanternFish { timer: 8 });
        }
        return None;
    }
}

fn simple_simulation(input: &Vec<i32>, num_cycle: usize) -> Vec<LanternFish> {
    let mut cur_cycle: Vec<LanternFish> = input.iter().map(|n| LanternFish { timer: *n }).collect();
    for _cycle in 0..num_cycle {
        let mut new_fish: Vec<LanternFish> = Vec::new();
        for fish in cur_cycle.iter_mut() {
            let result = fish.cycle();
            match result {
                Some(x) => new_fish.push(x),
                None => (),
            }
        }
        cur_cycle.append(&mut new_fish);
    }
    return cur_cycle;
}

fn array_simulation(input: &Vec<i32>, num_cycle: usize) -> usize {
    // instead of simulating each fish as an object, use a simple array of numbers
    let mut fish: [usize; 7] = [0; 7];
    let mut new_fish: [usize; 7] = [0; 7];
    for x in input {
        // initial seeding
        let timer: usize = (*x).try_into().unwrap();
        fish[timer] += 1;
    }
    for day in 0..num_cycle {
        // the fish on this timer are going to be the ones that do anything
        let num_fish_new = fish[day % 7];
        // There are essentially two fields we need to worry about here
        // (day + 2 ) % 7 is when these new fish should be considered, basically
        // 2 days after this fish, this will be added to to the fish to worry about.
        new_fish[(day + 2) % 7] = num_fish_new;
        // Update the number of fish on the next iteration of this value, with
        // thew number of new fish to care about.
        fish[day % 7] += new_fish[day % 7];
        // Clear out the number here to indicate we have used it.
        new_fish[day % 7] = 0;
    }
    let mut num_fish: usize = fish.iter().sum::<usize>();
    num_fish += new_fish.iter().sum::<usize>();
    return num_fish;
}

/// Solution to the first puzzle.
///
/// Likely this can be mathmatically modelled, but instead, going to create fish
/// that manage themselves and simply count.
///
/// ```
/// let input = vec![3, 4, 3, 1, 2];
/// assert_eq!(day06::puzzle_a(&input), 5934);
/// ```
pub fn puzzle_a(input: &Vec<i32>) -> usize {
    let total_cycles = 80;
    let finished_sim = simple_simulation(input, total_cycles);
    return finished_sim.len();
}

/// Solution to the second puzzle.
///
/// 256 is way too slow, runs for a long time just to do this test. As such
/// we are switching to using an array simulation.
///
/// ```
/// let input = vec![3, 4, 3, 1, 2];
/// assert_eq!(day06::puzzle_b(&input), 26984457539);
/// ```
pub fn puzzle_b(input: &Vec<i32>) -> usize {
    let total_cycles = 256;
    let finished_sim = array_simulation(input, total_cycles);
    return finished_sim;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let mut fish = LanternFish { timer: 1 };
        let result = fish.cycle();
        match result {
            Some(_x) => panic!("Fish made when not expected"),
            None => (),
        };
        assert_eq!(fish.timer, 0);
        let result2 = fish.cycle();
        match result2 {
            Some(x) => assert_eq!(x.timer, 8),
            None => panic!("Failed to make a fish"),
        };
        assert_eq!(fish.timer, 6);
    }

    #[test]
    fn test_simple_simulation() {
        let start = vec![3, 4, 3, 1, 2];
        assert_eq!(simple_simulation(&start, 0).len(), 5);
        assert_eq!(simple_simulation(&start, 1).len(), 5);
        assert_eq!(simple_simulation(&start, 2).len(), 6);
        assert_eq!(simple_simulation(&start, 8).len(), 10);
        assert_eq!(simple_simulation(&start, 18).len(), 26);
    }

    #[test]
    fn test_array_simulation() {
        let start = vec![3, 4, 3, 1, 2];
        assert_eq!(array_simulation(&start, 0), 5);
        assert_eq!(array_simulation(&start, 1), 5);
        assert_eq!(array_simulation(&start, 2), 6);
        assert_eq!(array_simulation(&start, 8), 10);
        assert_eq!(array_simulation(&start, 18), 26);
        assert_eq!(array_simulation(&start, 80), 5934);
    }
}
