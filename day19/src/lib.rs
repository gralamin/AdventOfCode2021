pub use filelib::{load, split_lines_by_blanks};
use itertools::Itertools;
pub use rustc_hash::{FxHashMap, FxHashSet};
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};
use std::str::FromStr;

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

fn rot_x_repeatedly(v: IVec3, n: usize) -> IVec3 {
    let mut result = v;
    for _ in 0..n {
        result = result.rot_x();
    }
    return result;
}

fn rot_y_repeatedly(v: IVec3, n: usize) -> IVec3 {
    let mut result = v;
    for _ in 0..n {
        result = result.rot_y();
    }
    return result;
}

fn rot_z_repeatedly(v: IVec3, n: usize) -> IVec3 {
    let mut result = v;
    for _ in 0..n {
        result = result.rot_z();
    }
    return result;
}

/// This is the technical term for the 24 possible rotations
///
/// Basically this function takes all of my data from a scanner
/// And rotates and reflects all of it.
fn chiral_octahedral_symmetry(scanner_data: Vec<IVec3>) -> Vec<Vec<IVec3>> {
    let mut all: Vec<Vec<IVec3>> = Vec::new();

    // This creates 32 rotations, with some duplicates
    // But this is fast and easy to write.
    for x_rot in 0..=1 {
        for y_rot in 0..=3 {
            for z_rot in 0..=3 {
                let mut new: Vec<IVec3> = Vec::new();
                for i in scanner_data.iter() {
                    new.push(rot_z_repeatedly(
                        rot_y_repeatedly(rot_x_repeatedly(i.clone(), x_rot), y_rot),
                        z_rot,
                    ));
                }
                all.push(new);
            }
        }
    }

    // Eliminate the duplicates.
    all = all.into_iter().unique().collect();

    return all;
}

/// Parse the scanner text
///
/// ```
/// use day19::vec3;
/// let s = vec!["--- scanner 0 ---".to_string(), "-1,-1,1".to_string(), "-2,-2,2".to_string(), "-3,-3,3".to_string(), "-2,-3,1".to_string(), "5,6,-4".to_string(), "8,0,7".to_string()];
/// let expected = vec![vec3(-1, -1, 1), vec3(-2, -2, 2), vec3(-3, -3, 3), vec3(-2, -3, 1), vec3(5,6,-4), vec3(8, 0, 7)];
/// assert_eq!(day19::parse_scanner(&s), (0, expected));
/// ```
pub fn parse_scanner(s: &Vec<String>) -> (usize, Vec<IVec3>) {
    let id = s.iter().nth(0).unwrap().split(' ').nth(2).unwrap().parse();
    let pos = s.iter().skip(1).map(|s| s.parse().unwrap()).collect();
    return (id.unwrap(), pos);
}

/// Figure out how many beacones there are
///
/// Test for this elsewhere, because this is so complicated of an input.
pub fn puzzle_a(scanner_map: &FxHashMap<usize, Vec<IVec3>>) -> usize {
    // Cache all the possible orientations
    let cache: Vec<Vec<Vec<IVec3>>> = scanner_map
        .iter()
        .map(|(_, points)| chiral_octahedral_symmetry(points.to_vec()))
        .collect();

    let map_of_beacons: Vec<_> = build_beacon_map(cache);

    return map_of_beacons.len();
}

// Okay, so to build the map
// We look for pairs at a time, and each time we find a pair
// that matches 12 common points on the same rotation, we combine them into a new scanner
// These new scanners, and any unmatched scanners go into the next loop
// Repeat until we have a single scanner
// That is the map relative to that scanner.
// To check if points match, we are going to choose a beacon in the left of the pair,
// and translate each point on the right pair such that it matches. Then check if we have 12 matches.
// If we do have 12 matches, thats it! If we don't choose the next point on the left.
fn build_beacon_map(mut cache: Vec<Vec<Vec<IVec3>>>) -> Vec<IVec3> {
    // Cache structure:
    // first vec is each scanner
    // second vec is each possible orientation of that scanner
    // third vec is the position of the becaons at that orientation

    // We decide first scanner is current origin.
    let mut beacons_at_origin = cache[0][0].clone();
    cache.remove(0);
    while cache.len() > 0 {
        // We have ideas of how to optimize right now, but just brute force and see how slow it is.
        let mut used = cache.len();
        for (i, point_orientations) in cache.iter().enumerate() {
            if let Some(result) =
                find_overlap_with(beacons_at_origin.clone(), point_orientations.to_vec())
            {
                beacons_at_origin = result;
                used = i;
                break;
            }
        }
        if used != cache.len() {
            // We found a result, get rid of used.
            cache.remove(used);
        }
    }

    return beacons_at_origin;
}

// Return the overlapped values all at origin 0 (beacons_at_origin's, original location)
fn find_overlap_with(
    beacons_at_origin: Vec<IVec3>,
    point_orientations: Vec<Vec<IVec3>>,
) -> Option<Vec<IVec3>> {
    let mut result: Vec<IVec3> = Vec::new();
    for orientation in point_orientations.iter() {
        for i in 0..beacons_at_origin.len() {
            let (origin, beacons_at_nth) = relative_to_nth(&beacons_at_origin, i);
            for j in 0..orientation.len() {
                let (_j_translation, orientation_over_beacon) = relative_to_nth(&orientation, j);
                if let Some(points) = overlap(&beacons_at_nth, &orientation_over_beacon) {
                    result.extend(beacons_at_nth);
                    result.extend(points);
                    let map_back = |v: &IVec3| *v + origin;
                    result = result.iter().map(map_back).collect();
                    return Some(result);
                }
            }
        }
    }
    return None;
}

fn relative_to_nth(points: &Vec<IVec3>, n: usize) -> (IVec3, FxHashSet<IVec3>) {
    let mapper = |v: &IVec3| *v - *points.get(n).unwrap();
    let new_origin = mapper(&vec3(0, 0, 0));
    let moved_points = points.iter().map(mapper).collect();
    return (new_origin, moved_points);
}

fn overlap(lefts_points: &FxHashSet<IVec3>, right_points: &FxHashSet<IVec3>) -> Option<Vec<IVec3>> {
    let mut matches: Vec<IVec3> = Vec::new();
    let mut new_values: Vec<IVec3> = Vec::new();
    for right_point in right_points.iter() {
        if lefts_points.contains(right_point) {
            matches.push(right_point.clone());
        } else {
            new_values.push(right_point.clone());
        }
    }
    if matches.len() < 12 {
        return None;
    }
    return Some(new_values);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_rotations() {
        let mut start: Vec<IVec3> = Vec::new();
        start.push(vec3(-1, -1, 1));
        start.push(vec3(-2, -2, 2));
        start.push(vec3(-3, -3, 3));
        start.push(vec3(-2, -3, 1));
        start.push(vec3(5, 6, -4));
        start.push(vec3(8, 0, 7));

        let result = chiral_octahedral_symmetry(start);
        assert_eq!(result.len(), 24);

        // Examples that we want to check are present
        let mut ex1: Vec<IVec3> = Vec::new();
        ex1.push(vec3(1, -1, 1));
        ex1.push(vec3(2, -2, 2));
        ex1.push(vec3(3, -3, 3));
        ex1.push(vec3(2, -1, 3));
        ex1.push(vec3(-5, 4, -6));
        ex1.push(vec3(-8, -7, 0));

        let mut ex2: Vec<IVec3> = Vec::new();
        ex2.push(vec3(-1, -1, -1));
        ex2.push(vec3(-2, -2, -2));
        ex2.push(vec3(-3, -3, -3));
        ex2.push(vec3(-1, -3, -2));
        ex2.push(vec3(4, 6, 5));
        ex2.push(vec3(-7, 0, 8));

        let mut ex3: Vec<IVec3> = Vec::new();
        ex3.push(vec3(1, 1, -1));
        ex3.push(vec3(2, 2, -2));
        ex3.push(vec3(3, 3, -3));
        ex3.push(vec3(1, 3, -2));
        ex3.push(vec3(-4, -6, 5));
        ex3.push(vec3(7, 0, 8));

        let mut ex4: Vec<IVec3> = Vec::new();
        ex4.push(vec3(1, 1, 1));
        ex4.push(vec3(2, 2, 2));
        ex4.push(vec3(3, 3, 3));
        ex4.push(vec3(3, 1, 2));
        ex4.push(vec3(-6, -4, -5));
        ex4.push(vec3(0, 7, -8));

        let all_exs = vec![ex1, ex2, ex3, ex4];
        for example in all_exs {
            let mut found = false;
            for x in result.iter() {
                if vecs_equal(x, &example) {
                    found = true;
                    break;
                }
            }
            if !found {
                panic!("Unable to find {:?}", example);
            }
        }
    }

    fn vecs_equal(left: &Vec<IVec3>, right: &Vec<IVec3>) -> bool {
        for (i, x) in left.iter().enumerate() {
            if right[i].x != x.x || right[i].y != x.y || right[i].z != x.z {
                return false;
            }
        }
        return true;
    }

    #[test]
    fn test_puzzle_a() {
        let input = "--- scanner 0 ---\n404,-588,-901\n528,-643,409\n-838,591,734\n390,-675,-793\n-537,-823,-458\n-485,-357,347\n-345,-311,381\n-661,-816,-575\n-876,649,763\n-618,-824,-621\n553,345,-567\n474,580,667\n-447,-329,318\n-584,868,-557\n544,-627,-890\n564,392,-477\n455,729,728\n-892,524,684\n-689,845,-530\n423,-701,434\n7,-33,-71\n630,319,-379\n443,580,662\n-789,900,-551\n459,-707,401\n\n--- scanner 1 ---\n686,422,578\n605,423,415\n515,917,-361\n-336,658,858\n95,138,22\n-476,619,847\n-340,-569,-846\n567,-361,727\n-460,603,-452\n669,-402,600\n729,430,532\n-500,-761,534\n-322,571,750\n-466,-666,-811\n-429,-592,574\n-355,545,-477\n703,-491,-529\n-328,-685,520\n413,935,-424\n-391,539,-444\n586,-435,557\n-364,-763,-893\n807,-499,-711\n755,-354,-619\n553,889,-390\n\n--- scanner 2 ---\n649,640,665\n682,-795,504\n-784,533,-524\n-644,584,-595\n-588,-843,648\n-30,6,44\n-674,560,763\n500,723,-460\n609,671,-379\n-555,-800,653\n-675,-892,-343\n697,-426,-610\n578,704,681\n493,664,-388\n-671,-858,530\n-667,343,800\n571,-461,-707\n-138,-166,112\n-889,563,-600\n646,-828,498\n640,759,510\n-630,509,768\n-681,-892,-333\n673,-379,-804\n-742,-814,-386\n577,-820,562\n\n--- scanner 3 ---\n-589,542,597\n605,-692,669\n-500,565,-823\n-660,373,557\n-458,-679,-417\n-488,449,543\n-626,468,-788\n338,-750,-386\n528,-832,-391\n562,-778,733\n-938,-730,414\n543,643,-506\n-524,371,-870\n407,773,750\n-104,29,83\n378,-903,-323\n-778,-728,485\n426,699,580\n-438,-605,-362\n-469,-447,-387\n509,732,623\n647,635,-688\n-868,-804,481\n614,-800,639\n595,780,-596\n\n--- scanner 4 ---\n727,592,562\n-293,-554,779\n441,611,-461\n-714,465,-776\n-743,427,-804\n-660,-479,-426\n832,-632,460\n927,-485,-438\n408,393,-506\n466,436,-512\n110,16,151\n-258,-428,682\n-393,719,612\n-211,-452,876\n808,-476,-593\n-575,615,604\n-485,667,467\n-680,325,-822\n-627,-443,-432\n872,-547,-609\n833,512,582\n807,604,487\n839,-516,451\n891,-625,532\n-652,-548,-490\n30,-46,-14";
        let split = split_lines_by_blanks(&input);
        let scanner_map: FxHashMap<usize, Vec<IVec3>> = split.iter().map(|x| parse_scanner(x)).collect();

        let result = puzzle_a(&scanner_map);
        assert_eq!(result, 79);
    }
}
