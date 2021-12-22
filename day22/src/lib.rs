pub use filelib::load;
pub use rustc_hash::FxHashMap;
use std::cmp::{max, min};
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};
use std::str::FromStr;

const CUBE_ON: bool = true;
const CUBE_OFF: bool = false;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct IVec3 {
    x: i32,
    y: i32,
    z: i32,
}

pub fn vec3(x: i32, y: i32, z: i32) -> IVec3 {
    return IVec3 { x: x, y: y, z: z };
}

// Seems that the fxhash is slower than this:
impl Hash for IVec3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x ^ self.y ^ self.z);
    }
}

impl IVec3 {
    fn rot_x(self) -> Self {
        return Self {
            x: self.x,
            y: -self.z,
            z: self.y,
        };
    }

    const fn rot_y(self) -> Self {
        return Self {
            x: -self.z,
            y: self.y,
            z: self.x,
        };
    }

    const fn rot_z(self) -> Self {
        return Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        };
    }

    fn dist_to(&self, other: &IVec3) -> u32 {
        return (*self - *other).dist();
    }

    fn dist(&self) -> u32 {
        return (self.x.abs() + self.y.abs() + self.z.abs())
            .try_into()
            .unwrap();
    }
}

impl Debug for IVec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Sub for IVec3 {
    type Output = IVec3;

    fn sub(self, other: IVec3) -> IVec3 {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Add for IVec3 {
    type Output = IVec3;

    fn add(self, other: IVec3) -> IVec3 {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl FromStr for IVec3 {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<IVec3, Infallible> {
        let mut s = s.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        let z = s.next().unwrap().parse().unwrap();
        return Ok(IVec3 { x, y, z });
    }
}

#[derive(Debug)]
pub enum Instruction {
    On(IVec3, IVec3),
    Off(IVec3, IVec3),
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (&Instruction::On(a, b), &Instruction::On(c, d)) => a == c && b == d,
            (&Instruction::Off(a, b), &Instruction::Off(c, d)) => a == c && b == d,
            _ => false,
        };
    }
}

/// Parse instructions from input
///
/// ```
/// use day22::vec3;
/// let s = "on x=10..12,y=10..12,z=10..12\non x=11..13,y=11..13,z=11..13\noff x=9..11,y=9..11,z=9..11\non x=10..10,y=10..10,z=10..10";
/// let v = day22::parse_instructions(s);
/// let expected = vec![
///      day22::Instruction::On(vec3(12, 12, 12), vec3(10, 10, 10)),
///      day22::Instruction::On(vec3(13, 13, 13), vec3(11, 11, 11)),
///      day22::Instruction::Off(vec3(11, 11, 11), vec3(9, 9, 9)),
///      day22::Instruction::On(vec3(10, 10, 10), vec3(10, 10, 10)),
/// ];
/// assert_eq!(v, expected);
/// ```
pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    let possible_instructions: Vec<Option<Instruction>> = input
        .lines()
        .map(|x| {
            let (type_str, coords_str) = x.split_once(" ").unwrap();
            let (x_str, yz_str) = coords_str.split_once(",").unwrap();
            let (y_str, z_str) = yz_str.split_once(",").unwrap();

            let (x_a_str, x_b_str) = x_str.split_once("..").unwrap();
            let (y_a_str, y_b_str) = y_str.split_once("..").unwrap();
            let (z_a_str, z_b_str) = z_str.split_once("..").unwrap();

            let (_, x_a_str) = x_a_str.split_once("=").unwrap();
            let (_, y_a_str) = y_a_str.split_once("=").unwrap();
            let (_, z_a_str) = z_a_str.split_once("=").unwrap();

            let x_a: i32 = x_a_str.parse().unwrap();
            let y_a: i32 = y_a_str.parse().unwrap();
            let z_a: i32 = z_a_str.parse().unwrap();

            let x_b: i32 = x_b_str.parse().unwrap();
            let y_b: i32 = y_b_str.parse().unwrap();
            let z_b: i32 = z_b_str.parse().unwrap();

            let x1 = max(x_a, x_b);
            let x2 = min(x_a, x_b);
            let y1 = max(y_a, y_b);
            let y2 = min(y_a, y_b);
            let z1 = max(z_a, z_b);
            let z2 = min(z_a, z_b);

            let upper_bound = vec3(x1, y1, z1);
            let lower_bound = vec3(x2, y2, z2);

            if type_str == "on" {
                return Some(Instruction::On(upper_bound, lower_bound));
            } else if type_str == "off" {
                return Some(Instruction::Off(upper_bound, lower_bound));
            }
            return None;
        })
        .collect();
    return possible_instructions
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
}

fn set_map(
    cube_map: &mut FxHashMap<IVec3, bool>,
    upper_bound: &IVec3,
    lower_bound: &IVec3,
    value: &bool,
) {
    for (x, y, z) in itertools::iproduct!(
        lower_bound.x..=upper_bound.x,
        lower_bound.y..=upper_bound.y,
        lower_bound.z..=upper_bound.z
    ) {
        let v = vec3(x, y, z);
        *cube_map.entry(v).or_insert(CUBE_OFF) = *value;
    }
}

fn set_map_only_with_50_cube(
    cube_map: &mut FxHashMap<IVec3, bool>,
    upper_bound: &IVec3,
    lower_bound: &IVec3,
    value: &bool,
) {
    let lower_x = max(lower_bound.x, -50);
    let lower_y = max(lower_bound.y, -50);
    let lower_z = max(lower_bound.z, -50);

    let upper_x = min(upper_bound.x, 50);
    let upper_y = min(upper_bound.y, 50);
    let upper_z = min(upper_bound.z, 50);

    for (x, y, z) in itertools::iproduct!(
        lower_x..=upper_x,
        lower_y..=upper_y,
        lower_z..=upper_z
    ) {
        let v = vec3(x, y, z);
        *cube_map.entry(v).or_insert(CUBE_OFF) = *value;
    }
}

fn within_cube(pos: &IVec3, upper_bound: &IVec3, lower_bound: &IVec3) -> bool {
    return pos.x <= upper_bound.x
        && lower_bound.x <= pos.x
        && pos.y <= upper_bound.y
        && lower_bound.y <= pos.y
        && pos.z <= upper_bound.z
        && lower_bound.z <= pos.z;
}

/// Run the boot up sequeunce, and count only cubes within -50 to 50 range.
///
/// ```
/// let s = "on x=10..12,y=10..12,z=10..12\non x=11..13,y=11..13,z=11..13\noff x=9..11,y=9..11,z=9..11\non x=10..10,y=10..10,z=10..10";
/// let ins = day22::parse_instructions(s);
/// assert_eq!(day22::puzzle_a(&ins), 39);
/// ```
pub fn puzzle_a(ins: &Vec<Instruction>) -> u128 {
    let lower_bound_final = vec3(-50, -50, -50);
    let upper_bound_final = vec3(50, 50, 50);
    let mut cube_map: FxHashMap<IVec3, bool> = FxHashMap::default();

    for instruction in ins {
        match instruction {
            Instruction::On(upper_bound, lower_bound) => {
                set_map_only_with_50_cube(&mut cube_map, upper_bound, lower_bound, &CUBE_ON)
            }
            Instruction::Off(upper_bound, lower_bound) => {
                set_map_only_with_50_cube(&mut cube_map, upper_bound, lower_bound, &CUBE_OFF)
            }
        }
    }

    let mut num_on: u128 = 0;
    for (location, state) in &cube_map {
        if within_cube(&location, &upper_bound_final, &lower_bound_final) && *state == CUBE_ON {
            num_on += 1;
        }
    }

    return num_on;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_within_cube() {
        let pos = vec3(10, 10, 10);
        let upper_bound = vec3(50, 50, 50);
        let lower_bound = vec3(-50, -50, -50);
        assert_eq!(within_cube(&pos, &upper_bound, &lower_bound), true);
        assert_eq!(within_cube(&lower_bound, &upper_bound, &lower_bound), true);
        assert_eq!(within_cube(&upper_bound, &upper_bound, &lower_bound), true);

        let out_pos = vec3(-100, -100, -100);
        assert_eq!(within_cube(&out_pos, &upper_bound, &lower_bound), false);
    }
}