use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc::input::parse_input_vec;
use cube::Cube;

use crate::face::{Axis, Face};

mod cube;
mod face;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(18, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Cube]) -> usize {
    // Convert cubes to faces
    let cube_faces = input.iter().flat_map(|c| c.to_faces()).collect::<Vec<_>>();

    // Create hash set of faces counting number of times a face is referenced
    let mut set = HashMap::new();

    for f in cube_faces {
        set.entry(f).and_modify(|e| *e += 1).or_insert(1);
    }

    // Count singly-owned faces
    set.into_iter().filter(|(_, cnt)| *cnt == 1).count()
}

fn part2(input: &[Cube]) -> usize {
    // Convert to set
    let cube_set = input.iter().cloned().collect::<HashSet<Cube>>();

    // Convert cubes to hash set of faces
    let cube_faces = input
        .iter()
        .flat_map(|c| c.to_faces())
        .collect::<HashSet<_>>();

    // Calculate min and max x, y and z
    let (min_x, max_x, min_y, max_y, min_z, max_z) = face_axis_min_max(&cube_faces);

    // Function to return a vector of cubes between two faces filtering out cubes in the input cube set
    let cubes = |f: &Face, p1, p2| {
        (p1..p2)
            .map(|pos| f.to_cube_at(pos))
            .filter(|c| !cube_set.contains(c))
            .collect::<Vec<_>>()
    };

    // Set of potential interior cubes
    let mut potential_interior = HashSet::new();

    // Find potential interior cubes
    for f in cube_faces.iter() {
        // Work out min and max range for the axis of this face
        let (min, max) = match f.axis() {
            Axis::X => (min_x, max_x),
            Axis::Y => (min_y, max_y),
            Axis::Z => (min_z, max_z),
        };

        // Clone the face for lookup
        let mut lookup_face = f.clone();

        // Scan downwards from this face
        for pos in (min..f.pos).rev() {
            // Is there a face at this position?
            lookup_face.pos = pos;

            if cube_faces.contains(&lookup_face) {
                // Yes - create a vector of all the spaces in between the faces and add to the potential interior set
                let cubes = cubes(f, pos, f.pos);
                potential_interior.extend(cubes);
                break;
            }
        }

        // Scan upwards from this face
        for pos in (f.pos + 1)..=max {
            // Is there a face at this position?
            lookup_face.pos = pos;

            if cube_faces.contains(&lookup_face) {
                // Yes - create a vector of all the spaces in between the faces and add to the potential interior set
                let cubes = cubes(f, f.pos, pos);
                potential_interior.extend(cubes);
                break;
            }
        }
    }

    // Set of actual interior cubes
    let mut interior;

    loop {
        // Function to check if the cube is real or potentially interior
        let check_cube = |c: Cube| cube_set.contains(&c) || potential_interior.contains(&c);

        // Scan each potential interior cube and check if is completely surrounded
        // by real cubes or other potential interior cubes
        interior = potential_interior
            .iter()
            .filter(|c| {
                check_cube(Cube::new_from_adj(c, -1, 0, 0))
                    && check_cube(Cube::new_from_adj(c, 1, 0, 0))
                    && check_cube(Cube::new_from_adj(c, 0, 0 - 1, 0))
                    && check_cube(Cube::new_from_adj(c, 0, 1, 0))
                    && check_cube(Cube::new_from_adj(c, 0, 0, -1))
                    && check_cube(Cube::new_from_adj(c, 0, 0, 1))
            })
            .cloned()
            .collect::<HashSet<_>>();

        // If we didn't eliminate any interior cubes we've finished
        if potential_interior.len() == interior.len() {
            break;
        }

        // Set potential to the current interior list
        potential_interior = interior;
    }

    // Convert the real cubes and interior cubes to a vector of faces
    let comb_faces = input
        .iter()
        .chain(interior.iter())
        .flat_map(|c| c.to_faces())
        .collect::<Vec<_>>();

    // Create hash set of faces and count references to each face
    let mut comb_set = HashMap::new();

    for f in comb_faces {
        comb_set.entry(f).and_modify(|e| *e += 1).or_insert(1);
    }

    // Count singly-owned faces
    comb_set.into_iter().filter(|(_, cnt)| *cnt == 1).count()
}

fn face_axis_min_max(cube_faces: &HashSet<Face>) -> (isize, isize, isize, isize, isize, isize) {
    cube_faces.iter().fold(
        (
            isize::MAX,
            isize::MIN,
            isize::MAX,
            isize::MIN,
            isize::MAX,
            isize::MIN,
        ),
        |(min_x, max_x, min_y, max_y, min_z, max_z), f| match f.axis() {
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
    )
}

// Input parsing

fn input_transform(line: String) -> Cube {
    let nums = line
        .split(',')
        .map(|s| {
            s.parse::<isize>()
                .unwrap_or_else(|_| panic!("Invalid number {}", s))
        })
        .collect::<Vec<_>>();

    Cube::new(nums[0], nums[1], nums[2])
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
