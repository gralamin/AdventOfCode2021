pub use filelib::load;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Variables {
    W,
    X,
    Y,
    Z,
}

impl std::str::FromStr for Variables {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "w" => Ok(Variables::W),
            "x" => Ok(Variables::X),
            "y" => Ok(Variables::Y),
            "z" => Ok(Variables::Z),
            _ => Err(()),
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum VariableOrInteger {
    Variable(Variables),
    Integer(i32),
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Instruction {
    Input(Variables),
    Add(Variables, VariableOrInteger),
    Mul(Variables, VariableOrInteger),
    Div(Variables, VariableOrInteger),
    Mod(Variables, VariableOrInteger),
    Eql(Variables, VariableOrInteger),
}

impl std::str::FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() == 1 {
            return Err("Unable to split");
        }
        let variable: Variables = parts.iter().nth(1).unwrap().parse().unwrap();
        let ins = parts.iter().nth(0).unwrap();
        if parts.len() == 2 {
            assert_eq!(*ins, "inp");
            return Ok(Instruction::Input(variable));
        } else {
            let as_int: Result<i32, _>;
            let as_variable: Result<Variables, _>;
            let b = parts.iter().nth(2).unwrap();
            as_int = b.parse();
            as_variable = b.parse();
            let b_parsed: VariableOrInteger;
            match as_int {
                Ok(v) => b_parsed = VariableOrInteger::Integer(v),
                Err(_) => match as_variable {
                    Ok(v) => b_parsed = VariableOrInteger::Variable(v),
                    Err(_) => return Err("Unable to pase b arg"),
                },
            }
            let instruction: Instruction;

            match *ins {
                "add" => instruction = Instruction::Add(variable, b_parsed),
                "mul" => instruction = Instruction::Mul(variable, b_parsed),
                "div" => instruction = Instruction::Div(variable, b_parsed),
                "mod" => instruction = Instruction::Mod(variable, b_parsed),
                "eql" => instruction = Instruction::Eql(variable, b_parsed),
                _ => return Err("Unable to parse instruction"),
            };

            return Ok(instruction);
        }
    }
}

impl Instruction {
    fn run(&self, state: &mut ALUState, input: &mut InputSupplier) {
        match self {
            Instruction::Input(v) => {
                let i: i32 = input.next();
                assert!(i < 10);
                assert!(i > 0);
                state.set_variable(v, i);
            }
            Instruction::Add(v, VariableOrInteger::Integer(i)) => {
                state.set_variable(v, state.get(v) + i);
            }
            Instruction::Add(v, VariableOrInteger::Variable(u)) => {
                state.set_variable(v, state.get(v) + state.get(u));
            }
            Instruction::Mul(v, VariableOrInteger::Integer(i)) => {
                state.set_variable(v, state.get(v) * i);
            }
            Instruction::Mul(v, VariableOrInteger::Variable(u)) => {
                state.set_variable(v, state.get(v) * state.get(u));
            }
            Instruction::Div(v, VariableOrInteger::Integer(i)) => {
                state.set_variable(v, state.get(v) / i);
            }
            Instruction::Div(v, VariableOrInteger::Variable(u)) => {
                state.set_variable(v, state.get(v) / state.get(u));
            }
            Instruction::Mod(v, VariableOrInteger::Integer(i)) => {
                state.set_variable(v, state.get(v) % i);
            }
            Instruction::Mod(v, VariableOrInteger::Variable(u)) => {
                state.set_variable(v, state.get(v) % state.get(u));
            }
            Instruction::Eql(v, VariableOrInteger::Integer(i)) => {
                state.set_variable(v, if state.get(v) == *i { 1 } else { 0 });
            }
            Instruction::Eql(v, VariableOrInteger::Variable(u)) => {
                state.set_variable(v, if state.get(v) == state.get(u) { 1 } else { 0 });
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct ALUState {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl ALUState {
    fn new() -> Self {
        return ALUState {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        };
    }

    fn get(&self, v: &Variables) -> i32 {
        return match v {
            Variables::W => self.w,
            Variables::X => self.x,
            Variables::Y => self.y,
            Variables::Z => self.z,
        };
    }

    fn set_variable(&mut self, v: &Variables, i: i32) {
        match v {
            Variables::W => self.w = i,
            Variables::X => self.x = i,
            Variables::Y => self.y = i,
            Variables::Z => self.z = i,
        }
    }
}

// Provides interface for Input operation
#[derive(Debug)]
struct InputSupplier {
    nums: Vec<i32>,
}

impl InputSupplier {
    fn from(i: u32) -> Self {
        let nums = i
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i32)
            .rev()
            .collect();
        return InputSupplier { nums: nums };
    }

    fn next(&mut self) -> i32 {
        if self.nums.len() > 0 {
            return self.nums.pop().unwrap();
        }
        return 999;
    }
}

/// Parse the string into instructions, ignoring blanks
///
/// ```
/// use day24::Instruction::{Input, Add, Mod, Div, Eql};
/// use day24::Variables::{W, X, Y, Z};
/// use day24::VariableOrInteger::{Variable, Integer};
/// let instructions = "inp w\nadd z w\nmod z 2\ndiv w 2\nadd y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2\n";
/// let expected = vec![
///     Input(W),
///     Add(Z, Variable(W)),
///     Mod(Z, Integer(2)),
///     Div(W, Integer(2)),
///     Add(Y, Variable(W)),
///     Mod(Y, Integer(2)),
///     Div(W, Integer(2)),
///     Add(X, Variable(W)),
///     Mod(X, Integer(2)),
///     Div(W, Integer(2)),
///     Mod(W, Integer(2))
/// ];
/// assert_eq!(day24::parse_all_instrcutions(instructions), expected);
/// ```
pub fn parse_all_instrcutions(s: &str) -> Vec<Instruction> {
    return s
        .lines()
        .filter_map(|x| x.parse::<Instruction>().ok())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_instruction_input() {
        let s = "inp x";
        let expected = Instruction::Input(Variables::X);
        let result: Instruction = s.parse().unwrap();
        assert_eq!(result, expected);

        let s = "inp z";
        let expected = Instruction::Input(Variables::Z);
        let result: Instruction = s.parse().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_empty_line() {
        let s = "";
        let result: Result<Instruction, _> = s.parse();
        match result {
            Ok(v) => panic!("Should not get a value here"),
            Err(_) => assert_eq!(1, 1),
        };
    }

    #[test]
    fn test_parse_instruction_input() {
        let s = "add x y";
        let expected = Instruction::Add(Variables::X, VariableOrInteger::Variable(Variables::Y));
        let result: Instruction = s.parse().unwrap();
        assert_eq!(result, expected);

        let s = "mul w -5";
        let expected = Instruction::Mul(Variables::W, VariableOrInteger::Integer(-5));
        let result: Instruction = s.parse().unwrap();
        assert_eq!(result, expected);

        let s = "div w 2";
        let expected = Instruction::Div(Variables::W, VariableOrInteger::Integer(2));
        let result: Instruction = s.parse().unwrap();
        assert_eq!(result, expected);

        let s = "mod y x";
        let expected = Instruction::Mod(Variables::Y, VariableOrInteger::Variable(Variables::X));
        let result: Instruction = s.parse().unwrap();
        assert_eq!(result, expected);

        let s = "eql w 2";
        let expected = Instruction::Eql(Variables::W, VariableOrInteger::Integer(2));
        let result: Instruction = s.parse().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_program() {
        let mut state = ALUState::new();
        let mut inputs = InputSupplier::from(51);
        let instructions = "inp w\nadd z w\nmod z 2\ndiv w 2\nadd y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2\n";
        let parsed_instructions = parse_all_instrcutions(instructions);
        for instruction in parsed_instructions {
            instruction.run(&mut state, &mut inputs);
        }
        assert_eq!(state.w, 0);
        assert_eq!(state.x, 1);
        assert_eq!(state.y, 0);
        assert_eq!(state.z, 1);
    }
}
