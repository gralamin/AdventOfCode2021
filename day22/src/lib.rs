pub use filelib::load;
pub use ivec3::{vec3, IVec3};
pub use rustc_hash::FxHashMap;
use std::cmp::{max, min};

const CUBE_ON: bool = true;
const CUBE_OFF: bool = false;

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

    for (x, y, z) in itertools::iproduct!(lower_x..=upper_x, lower_y..=upper_y, lower_z..=upper_z) {
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

#[derive(Debug, Clone, Copy)]
struct Cube {
    upper_bound: IVec3,
    lower_bound: IVec3,
}

impl Cube {
    fn new(upper_bound: IVec3, lower_bound: IVec3) -> Self {
        return Self {
            upper_bound: upper_bound,
            lower_bound: lower_bound,
        };
    }

    fn volume(&self) -> u128 {
        let bound_math = self.upper_bound - self.lower_bound + vec3(1, 1, 1);
        return bound_math.x as u128 * bound_math.y as u128 * bound_math.z as u128;
    }

    fn has_overlap(&self, other: &Cube) -> bool {
        if self.lower_bound.x > other.upper_bound.x || self.upper_bound.x < other.lower_bound.x {
            // X's are distinct
            return false;
        }
        if self.lower_bound.y > other.upper_bound.y || self.upper_bound.y < other.lower_bound.y {
            // X's are distinct
            return false;
        }
        if self.lower_bound.z > other.upper_bound.z || self.upper_bound.z < other.lower_bound.z {
            // X's are distinct
            return false;
        }
        return true;
    }

    fn split_overlapping_cubes(&self, other: &Cube) -> Vec<Cube> {
        let mut new_cubes = Vec::new();

        if other.lower_bound.x > self.lower_bound.x {
            let new_lower_bound = vec3(self.lower_bound.x, self.lower_bound.y, self.lower_bound.z);
            let new_upper_bound = vec3(
                other.lower_bound.x - 1,
                self.upper_bound.y,
                self.upper_bound.z,
            );
            new_cubes.push(Cube::new(new_upper_bound, new_lower_bound));
        }
        if other.upper_bound.x < self.upper_bound.x {
            let new_lower_bound = vec3(
                other.upper_bound.x + 1,
                self.lower_bound.y,
                self.lower_bound.z,
            );
            let new_upper_bound = vec3(self.upper_bound.x, self.upper_bound.y, self.upper_bound.z);
            new_cubes.push(Cube::new(new_upper_bound, new_lower_bound));
        }
        if other.lower_bound.y > self.lower_bound.y {
            let new_lower_bound = vec3(
                max(self.lower_bound.x, other.lower_bound.x),
                self.lower_bound.y,
                self.lower_bound.z,
            );
            let new_upper_bound = vec3(
                min(self.upper_bound.x, other.upper_bound.x),
                other.lower_bound.y - 1,
                self.upper_bound.z,
            );
            new_cubes.push(Cube::new(new_upper_bound, new_lower_bound));
        }
        if other.upper_bound.y < self.upper_bound.y {
            let new_lower_bound = vec3(
                max(self.lower_bound.x, other.lower_bound.x),
                other.upper_bound.y + 1,
                self.lower_bound.z,
            );
            let new_upper_bound = vec3(
                min(self.upper_bound.x, other.upper_bound.x),
                self.upper_bound.y,
                self.upper_bound.z,
            );
            new_cubes.push(Cube::new(new_upper_bound, new_lower_bound));
        }
        if other.lower_bound.z > self.lower_bound.z {
            let new_lower_bound = vec3(
                max(self.lower_bound.x, other.lower_bound.x),
                max(self.lower_bound.y, other.lower_bound.y),
                self.lower_bound.z,
            );
            let new_upper_bound = vec3(
                min(self.upper_bound.x, other.upper_bound.x),
                min(self.upper_bound.y, other.upper_bound.y),
                other.lower_bound.z - 1,
            );
            new_cubes.push(Cube::new(new_upper_bound, new_lower_bound));
        }
        if other.upper_bound.z < self.upper_bound.z {
            let new_lower_bound = vec3(
                max(self.lower_bound.x, other.lower_bound.x),
                max(self.lower_bound.y, other.lower_bound.y),
                other.upper_bound.z + 1,
            );
            let new_upper_bound = vec3(
                min(self.upper_bound.x, other.upper_bound.x),
                min(self.upper_bound.y, other.upper_bound.y),
                self.upper_bound.z,
            );
            new_cubes.push(Cube::new(new_upper_bound, new_lower_bound));
        }

        return new_cubes;
    }
}

/// Run the boot up sequeunce, and count cubes within a range.
///
/// ```
/// let s = "on x=-5..47,y=-31..22,z=-19..33\non x=-44..5,y=-27..21,z=-14..35\non x=-49..-1,y=-11..42,z=-10..38\non x=-20..34,y=-40..6,z=-44..1\noff x=26..39,y=40..50,z=-2..11\non x=-41..5,y=-41..6,z=-36..8\noff x=-43..-33,y=-45..-28,z=7..25\non x=-33..15,y=-32..19,z=-34..11\noff x=35..47,y=-46..-34,z=-11..5\non x=-14..36,y=-6..44,z=-16..29\non x=-57795..-6158,y=29564..72030,z=20435..90618\non x=36731..105352,y=-21140..28532,z=16094..90401\non x=30999..107136,y=-53464..15513,z=8553..71215\non x=13528..83982,y=-99403..-27377,z=-24141..23996\non x=-72682..-12347,y=18159..111354,z=7391..80950\non x=-1060..80757,y=-65301..-20884,z=-103788..-16709\non x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\non x=-52752..22273,y=-49450..9096,z=54442..119054\non x=-29982..40483,y=-108474..-28371,z=-24328..38471\non x=-4958..62750,y=40422..118853,z=-7672..65583\non x=55694..108686,y=-43367..46958,z=-26781..48729\non x=-98497..-18186,y=-63569..3412,z=1232..88485\non x=-726..56291,y=-62629..13224,z=18033..85226\non x=-110886..-34664,y=-81338..-8658,z=8914..63723\non x=-55829..24974,y=-16897..54165,z=-121762..-28058\non x=-65152..-11147,y=22489..91432,z=-58782..1780\non x=-120100..-32970,y=-46592..27473,z=-11695..61039\non x=-18631..37533,y=-124565..-50804,z=-35667..28308\non x=-57817..18248,y=49321..117703,z=5745..55881\non x=14781..98692,y=-1341..70827,z=15753..70151\non x=-34419..55919,y=-19626..40991,z=39015..114138\non x=-60785..11593,y=-56135..2999,z=-95368..-26915\non x=-32178..58085,y=17647..101866,z=-91405..-8878\non x=-53655..12091,y=50097..105568,z=-75335..-4862\non x=-111166..-40997,y=-71714..2688,z=5609..50954\non x=-16602..70118,y=-98693..-44401,z=5197..76897\non x=16383..101554,y=4615..83635,z=-44907..18747\noff x=-95822..-15171,y=-19987..48940,z=10804..104439\non x=-89813..-14614,y=16069..88491,z=-3297..45228\non x=41075..99376,y=-20427..49978,z=-52012..13762\non x=-21330..50085,y=-17944..62733,z=-112280..-30197\non x=-16478..35915,y=36008..118594,z=-7885..47086\noff x=-98156..-27851,y=-49952..43171,z=-99005..-8456\noff x=2032..69770,y=-71013..4824,z=7471..94418\non x=43670..120875,y=-42068..12382,z=-24787..38892\noff x=37514..111226,y=-45862..25743,z=-16714..54663\noff x=25699..97951,y=-30668..59918,z=-15349..69697\noff x=-44271..17935,y=-9516..60759,z=49131..112598\non x=-61695..-5813,y=40978..94975,z=8655..80240\noff x=-101086..-9439,y=-7088..67543,z=33935..83858\noff x=18020..114017,y=-48931..32606,z=21474..89843\noff x=-77139..10506,y=-89994..-18797,z=-80..59318\noff x=8476..79288,y=-75520..11602,z=-96624..-24783\non x=-47488..-1262,y=24338..100707,z=16292..72967\noff x=-84341..13987,y=2429..92914,z=-90671..-1318\noff x=-37810..49457,y=-71013..-7894,z=-105357..-13188\noff x=-27365..46395,y=31009..98017,z=15428..76570\noff x=-70369..-16548,y=22648..78696,z=-1892..86821\non x=-53470..21291,y=-120233..-33476,z=-44150..38147\noff x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
/// let ins = day22::parse_instructions(s);
/// assert_eq!(day22::puzzle_b(&ins), 2758514936282235);
/// ```
pub fn puzzle_b(ins: &Vec<Instruction>) -> u128 {
    // Can we just get away with making a bunch of cubes dynamically, and adding and subtracting the volume?
    // This won't work if a sub area of a cube is turned on, or off, multiple times in succession.
    let mut on_cubes: Vec<Option<Cube>> = Vec::new();
    for instruction in ins {
        let should_push: bool;
        let new_cube: Cube;
        match instruction {
            Instruction::On(upper_bound, lower_bound) => {
                new_cube = Cube::new(*upper_bound, *lower_bound);
                should_push = true;
            }
            Instruction::Off(upper_bound, lower_bound) => {
                new_cube = Cube::new(*upper_bound, *lower_bound);
                should_push = false;
            }
        }
        for i in 0..on_cubes.len() {
            let cube_at_index = on_cubes.get(i).unwrap();
            if let Some(cur_cube) = cube_at_index {
                if cur_cube.has_overlap(&new_cube) {
                    let mut split_cubes = cur_cube
                        .split_overlapping_cubes(&new_cube)
                        .into_iter()
                        .map(|x| Some(x))
                        .collect();
                    on_cubes[i] = None;
                    on_cubes.append(&mut split_cubes);
                }
            }
        }
        if should_push {
            on_cubes.push(Some(new_cube));
        }
        // Remove empty spaces.
        on_cubes = on_cubes.into_iter().filter(|x| x.is_some()).collect();
    }

    let mut num_on: u128 = 0;
    for cube in on_cubes {
        if let Some(cube) = cube {
            num_on += cube.volume();
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
