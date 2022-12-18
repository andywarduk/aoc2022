use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(18, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Coord]) -> usize {
    // Convert to faces
    let cube_faces = input.iter().flat_map(faces).collect::<Vec<_>>();

    // Create hash set of faces
    let mut set = HashMap::new();

    for f in cube_faces {
        set.entry(f).and_modify(|e| *e += 1).or_insert(1);
    }

    // Count singly-owned faces
    set.into_iter().filter(|(_, cnt)| *cnt == 1).count()
}

fn part2(input: &[Coord]) -> usize {
    // Convert to set
    let cube_set = input.iter().cloned().collect::<HashSet<Coord>>();

    // Convert to faces
    let cube_faces = input.iter().flat_map(faces).collect::<HashSet<_>>();

    let (min_x, max_x, min_y, max_y, min_z, max_z) = cube_faces.iter().fold(
        (usize::MAX, 0, usize::MAX, 0, usize::MAX, 0),
        |(min_x, max_x, min_y, max_y, min_z, max_z), f| match f.axis {
            Axis::X => (
                min(min_x, f.pos),
                max(max_x, f.pos),
                min_y,
                max_y,
                min_z,
                max_z,
            ),
            Axis::Y => (
                min_x,
                max_x,
                min(min_y, f.pos),
                max(max_y, f.pos),
                min_z,
                max_z,
            ),
            Axis::Z => (
                min_x,
                max_x,
                min_y,
                max_y,
                min(min_z, f.pos),
                max(max_z, f.pos),
            ),
        },
    );

    let mut potential_interior = HashSet::new();

    let cubes = |f: &Face, p1, p2| {
        (p1..p2)
            .map(|pos| match f.axis {
                Axis::X => Coord {
                    x: pos,
                    y: f.other[0],
                    z: f.other[1],
                },
                Axis::Y => Coord {
                    x: f.other[0],
                    y: pos,
                    z: f.other[1],
                },
                Axis::Z => Coord {
                    x: f.other[0],
                    y: f.other[1],
                    z: pos,
                },
            })
            .filter(|c| !cube_set.contains(c))
            .collect::<Vec<_>>()
    };

    for f in cube_faces.iter() {
        let (min, max) = match f.axis {
            Axis::X => (min_x, max_x),
            Axis::Y => (min_y, max_y),
            Axis::Z => (min_z, max_z),
        };

        let mut lookup_face = f.clone();

        for pos in (min..f.pos).rev() {
            lookup_face.pos = pos;

            if cube_faces.contains(&lookup_face) {
                let mut cubes = cubes(f, pos, f.pos);
                potential_interior.extend(cubes);
                break;
            }
        }

        for pos in (f.pos + 1)..=max {
            lookup_face.pos = pos;

            if cube_faces.contains(&lookup_face) {
                let mut cubes = cubes(f, f.pos, pos);
                potential_interior.extend(cubes);
                break;
            }
        }
    }

    let mut interior;

    loop {
        println!("potential: {:?}", potential_interior.len());

        let check_cube = |c: Coord| cube_set.contains(&c) || potential_interior.contains(&c);

        interior = potential_interior
            .iter()
            .filter(|c| {
                check_cube(Coord {
                    x: c.x - 1,
                    y: c.y,
                    z: c.z,
                }) && check_cube(Coord {
                    x: c.x + 1,
                    y: c.y,
                    z: c.z,
                }) && check_cube(Coord {
                    x: c.x,
                    y: c.y - 1,
                    z: c.z,
                }) && check_cube(Coord {
                    x: c.x,
                    y: c.y + 1,
                    z: c.z,
                }) && check_cube(Coord {
                    x: c.x,
                    y: c.y,
                    z: c.z - 1,
                }) && check_cube(Coord {
                    x: c.x,
                    y: c.y,
                    z: c.z + 1,
                })
            })
            .cloned()
            .collect::<HashSet<_>>();

        println!("interior: {:?}", interior.len());

        if potential_interior.len() == interior.len() {
            break;
        }

        potential_interior = interior;
    }

    println!("interior: {:?}", interior);

    // Convert to faces
    let comb_faces = input
        .iter()
        .chain(interior.iter())
        .flat_map(faces)
        .collect::<Vec<_>>();

    // Create hash set of faces
    let mut comb_set = HashMap::new();

    for f in comb_faces {
        comb_set.entry(f).and_modify(|e| *e += 1).or_insert(1);
    }

    // Count singly-owned faces
    comb_set.into_iter().filter(|(_, cnt)| *cnt == 1).count()
}

fn faces(c: &Coord) -> Vec<Face> {
    vec![
        Face {
            axis: Axis::X,
            pos: c.x,
            other: [c.y, c.z],
        },
        Face {
            axis: Axis::X,
            pos: c.x + 1,
            other: [c.y, c.z],
        },
        Face {
            axis: Axis::Y,
            pos: c.y,
            other: [c.x, c.z],
        },
        Face {
            axis: Axis::Y,
            pos: c.y + 1,
            other: [c.x, c.z],
        },
        Face {
            axis: Axis::Z,
            pos: c.z,
            other: [c.x, c.y],
        },
        Face {
            axis: Axis::Z,
            pos: c.z + 1,
            other: [c.x, c.y],
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Face {
    axis: Axis,
    pos: usize,
    other: [usize; 2],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

// Input parsing

fn input_transform(line: String) -> Coord {
    let nums = line
        .split(',')
        .map(|s| {
            s.parse::<usize>()
                .unwrap_or_else(|_| panic!("Invalid number {}", s))
        })
        .collect::<Vec<_>>();

    Coord {
        x: nums[0],
        y: nums[1],
        z: nums[2],
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 64);
        assert_eq!(part2(&input), 58);
    }
}
