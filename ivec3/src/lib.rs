use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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
    pub fn rot_x(self) -> Self {
        return Self {
            x: self.x,
            y: -self.z,
            z: self.y,
        };
    }

    pub fn rot_y(self) -> Self {
        return Self {
            x: -self.z,
            y: self.y,
            z: self.x,
        };
    }

    pub fn rot_z(self) -> Self {
        return Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        };
    }

    pub fn dist_to(&self, other: &IVec3) -> u32 {
        return (*self - *other).dist();
    }

    pub fn dist(&self) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_add() {
        let one = vec3(-3, -5, -7);
        let two = vec3(1, 9, 7);
        let expected = vec3(-2, 4, 0);
        assert_eq!(one + two, expected);
    }

    #[test]
    fn test_sub() {
        let one = vec3(-3, -5, -7);
        let two = vec3(1, -9, 7);
        let expected = vec3(-4, 4, -14);
        assert_eq!(one - two, expected);
    }

    #[test]
    fn test_from_string() {
        let s = "1,4,3";
        let result: IVec3 = s.parse().unwrap();
        assert_eq!(result, vec3(1, 4, 3));
    }

    #[test]
    fn test_can_hash() {
        let mut set: HashSet<IVec3> = HashSet::new();
        let one = vec3(1, 1, 1);
        let two = vec3(1, 0, 1);
        let three = vec3(0, 1, 1);
        let four = vec3(1, 1, 0);
        set.insert(one);
        set.insert(four);
        assert_eq!(set.len(), 2);
        assert_eq!(set.contains(&one), true);
        assert_eq!(set.contains(&two), false);
        assert_eq!(set.contains(&three), false);
        assert_eq!(set.contains(&four), true);
    }

    #[test]
    fn test_rot_x() {
        let a = vec3(1, 2, 4);
        assert_eq!(a.rot_x(), vec3(1, -4, 2));
    }

    #[test]
    fn test_rot_y() {
        let a = vec3(1, 2, 4);
        assert_eq!(a.rot_y(), vec3(-4, 2, 1));
    }

    #[test]
    fn test_rot_z() {
        let a = vec3(1, 2, 4);
        assert_eq!(a.rot_z(), vec3(2, -1, 4));
    }

    #[test]
    fn test_dist() {
        let x = vec3(1, -4, 5);
        assert_eq!(x.dist(), 10);
    }

    #[test]
    fn test_dist_to() {
        let x = vec3(1, -4, 5);
        let y = vec3(99, 99, 99);
        assert_eq!(x.dist_to(&y), 295);
    }
}
