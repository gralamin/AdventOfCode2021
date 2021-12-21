pub use filelib::load;
pub use rustc_hash::FxHashMap;

type CacheKey = Vec<u32>;
type CacheValue = Vec<u128>;
type Cache = FxHashMap<CacheKey, CacheValue>;

#[derive(Debug)]
struct DeterministicDie {
    cur_roll: u32,
    max_size: u32,
    num_rolls: u32,
}

impl DeterministicDie {
    fn new(sides: u32) -> Self {
        return Self {
            cur_roll: 0,
            max_size: sides,
            num_rolls: 0,
        };
    }

    fn roll(&mut self) -> u32 {
        self.cur_roll += 1;
        self.num_rolls += 1;
        let roll = self.cur_roll;
        self.cur_roll = self.cur_roll % self.max_size;
        return roll;
    }
}

fn roll_three_times(die: &mut DeterministicDie) -> u32 {
    let roll_one = die.roll();
    let roll_two = die.roll();
    let roll_three = die.roll();
    return roll_one + roll_two + roll_three;
}

#[derive(Debug)]
struct State<'a> {
    max_pos: u32,
    player_pos: Vec<u32>,
    cur_turn: usize,
    player_score: Vec<u32>,
    die: &'a mut DeterministicDie,
    victory: u32,
}

impl<'a> State<'a> {
    fn new(num_players: usize, max_pos: u32, die: &'a mut DeterministicDie) -> Self {
        let players = vec![0; num_players];
        let players_score = vec![0; num_players];
        return Self {
            max_pos: max_pos,
            player_pos: players,
            cur_turn: 0,
            player_score: players_score,
            die: die,
            victory: 1000,
        };
    }

    fn set_start_position(&mut self, pos: &Vec<u32>) {
        self.player_pos = pos.clone();
    }

    fn turn(&mut self) {
        let move_pos = roll_three_times(self.die);
        let start_pos = self.player_pos[self.cur_turn];
        let mut new_pos: u32 = move_pos + start_pos;
        while new_pos > self.max_pos {
            new_pos -= self.max_pos;
        }
        self.player_pos[self.cur_turn] = new_pos;

        self.player_score[self.cur_turn] += new_pos as u32;
        self.cur_turn += 1;
        self.cur_turn = self.cur_turn % self.player_score.len();
    }

    fn get_num_rolls(&self) -> u32 {
        return self.die.num_rolls;
    }

    fn get_lowest_score(&self) -> u32 {
        return *self.player_score.iter().min().unwrap();
    }

    fn game_is_end(&self) -> bool {
        let winning_players: Vec<&u32> = self
            .player_score
            .iter()
            .filter(|&x| *x >= self.victory)
            .collect();
        return winning_players.len() >= 1;
    }
}

#[derive(Debug, Clone)]
struct QuantumState {
    max_pos: u32,
    player_pos: Vec<u32>,
    cur_turn: usize,
    player_score: Vec<u32>,
    victory: u32,
}

impl QuantumState {
    fn new(num_players: usize, max_pos: u32) -> Self {
        let players = vec![0; num_players];
        let players_score = vec![0; num_players];
        return Self {
            max_pos: max_pos,
            player_pos: players,
            cur_turn: 0,
            player_score: players_score,
            victory: 21,
        };
    }

    fn set_start_position(&mut self, pos: &Vec<u32>) {
        self.player_pos = pos.clone();
    }

    fn turn(&self) -> Vec<QuantumState> {
        // So if we roll a 3 sided die 3 times, we have 27 realities
        let mut universes: Vec<QuantumState> = Vec::new();
        for first_roll_value in 1..=3 {
            for second_roll_value in 1..=3 {
                for third_roll_value in 1..=3 {
                    let mut new = self.clone();
                    new.advance_by_roll(first_roll_value + second_roll_value + third_roll_value);
                    // Ensure the Vecs are being copied, that could maybe cause my mystery state...
                    assert_ne!(self.player_pos, new.player_pos);
                    assert_ne!(self.player_score, new.player_score);
                    universes.push(new);
                }
            }
        }
        assert_eq!(universes.len(), 27);
        return universes;
    }

    fn advance_by_roll(&mut self, roll: usize) {
        let move_pos: u32 = roll as u32;
        let start_pos = self.player_pos[self.cur_turn];
        let mut new_pos: u32 = move_pos + start_pos;
        while new_pos > self.max_pos {
            new_pos -= self.max_pos;
        }
        self.player_pos[self.cur_turn] = new_pos;
        self.player_score[self.cur_turn] += new_pos as u32;
        self.cur_turn += 1;
        self.cur_turn = self.cur_turn % self.player_score.len();
    }

    fn game_is_end(&self) -> bool {
        let winning_players: Vec<&u32> = self
            .player_score
            .iter()
            .filter(|&x| *x >= self.victory)
            .collect();
        return winning_players.len() >= 1;
    }

    fn get_winner(&self) -> usize {
        for (i, score) in self.player_score.iter().enumerate() {
            if *score >= self.victory {
                return i;
            }
        }
        return usize::MAX;
    }

    fn to_cache_key(&self) -> CacheKey {
        let mut cache_key = Vec::new();
        cache_key.push(self.cur_turn as u32); // The cache must tell turns apart.
        cache_key.append(&mut self.player_pos.clone());
        cache_key.append(&mut self.player_score.clone());
        return cache_key;
    }
}

/// Parse the input format, such that player 1 is at index 0, 2 is at index 1, etc.
///
/// ```
/// let input = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
/// assert_eq!(day21::parse_player_pos(input), vec![4, 8]);
/// ```
pub fn parse_player_pos(input: &str) -> Vec<u32> {
    let mut output = Vec::new();
    for x in input.lines() {
        let (_, num_str) = x.split_once("position: ").unwrap();
        output.push(num_str.parse::<u32>().unwrap());
    }

    return output;
}

/// Run the game on a determinstic die, and return loser score * num rolls
///
/// ```
/// let player_pos: Vec<u32> = vec![4, 8];
/// assert_eq!(day21::puzzle_a(&player_pos), 739785);
/// ```
pub fn puzzle_a(player_pos: &Vec<u32>) -> u32 {
    let mut die = DeterministicDie::new(100);
    let mut state = State::new(player_pos.len(), 10, &mut die);
    state.set_start_position(player_pos);
    while !state.game_is_end() {
        state.turn();
    }
    let rolls = state.get_num_rolls();
    let low_score = state.get_lowest_score();
    return rolls * low_score;
}

fn run_one_quantum_game(state: &QuantumState, cache: &mut Cache) -> CacheValue {
    // If its in our cache, don't do all this again.
    if let Some(score) = cache.get(&state.to_cache_key()) {
        return score.clone();
    }

    let mut score: CacheValue = vec![0; state.player_pos.len()];
    // First check if this game is already done, if so toss it.
    if state.game_is_end() {
        let winner = state.get_winner();
        score[winner] += 1;
        return score;
    }

    // Recursively run the sub universes
    let new_universes = state.turn();
    for universe in new_universes.iter() {
        let winners = run_one_quantum_game(&universe, cache);
        for (i, v) in winners.iter().enumerate() {
            score[i] += v;
        }
    }

    // Insert into the cache before we leave, so we don't have to play these ones again.
    cache.insert(state.to_cache_key(), score.clone());
    return score;
}

/// Run the quantum game and return the higher number of wins.
///
/// In the example below results should be:
/// player 1: 444356092776315 wins
/// player 2: 341960390180808 wins
/// ```
/// let player_pos: Vec<u32> = vec![4, 8];
/// assert_eq!(day21::puzzle_b(&player_pos), 444356092776315);
/// ```
pub fn puzzle_b(player_pos: &Vec<u32>) -> u128 {
    let mut start_state = QuantumState::new(player_pos.len(), 10);
    start_state.set_start_position(&player_pos);
    let mut cache: Cache = FxHashMap::default();
    let result = run_one_quantum_game(&start_state, &mut cache);
    return *result.iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_determinstic_die() -> DeterministicDie {
        return DeterministicDie::new(100);
    }

    fn get_state(die: &mut DeterministicDie) -> State {
        let mut state = State::new(2, 10, die);
        state.set_start_position(&vec![4, 8]);
        return state;
    }

    #[test]
    fn test_roll_determinstic_die_101() {
        let mut die = get_determinstic_die();
        assert_eq!(die.roll(), 1);
        assert_eq!(die.roll(), 2);
        assert_eq!(die.roll(), 3);
        for i in 1..=97 {
            assert_eq!(die.roll(), 3 + i);
        }
        assert_eq!(die.roll(), 1);
    }

    #[test]
    fn test_get_num_rolls() {
        let mut die = get_determinstic_die();
        for _ in 0..300 {
            die.roll();
        }
        let state = get_state(&mut die);
        assert_eq!(state.get_num_rolls(), 300);
    }

    #[test]
    fn test_first_eight_turns() {
        let mut die = get_determinstic_die();
        let mut state = get_state(&mut die);
        for _ in 0..8 {
            state.turn();
        }
        assert_eq!(state.player_pos, vec![6, 6]);
        assert_eq!(state.player_score, vec![26, 22]);
    }
}
