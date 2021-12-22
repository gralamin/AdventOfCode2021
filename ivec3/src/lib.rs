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