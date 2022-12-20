use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(20, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[i64]) -> i64 {
    // Transform input
    let input = input
        .iter()
        .enumerate()
        .map(|(i, &n)| (i, n))
        .collect::<Vec<_>>();

    // Create work vector from input
    let mut vec = input.to_vec();

    // Mix it up
    mix(&input, &mut vec);

    // Calculate result
    elem(&vec, 1000) + elem(&vec, 2000) + elem(&vec, 3000)
}

fn part2(input: &[i64]) -> i64 {
    // Transform input
    let input = input
        .iter()
        .enumerate()
        .map(|(i, &n)| (i, n * 811589153))
        .collect::<Vec<_>>();

    // Create work vector from input
    let mut vec = input.to_vec();

    // Loop 10 times
    for _ in 0..10 {
        // Mix it up
        mix(&input, &mut vec);
    }

    // Calculate result
    elem(&vec, 1000) + elem(&vec, 2000) + elem(&vec, 3000)
}

fn mix(input: &[(usize, i64)], vec: &mut Vec<(usize, i64)>) {
    for val in input {
        // Get vector position of the number
        let old_pos = vec.iter().position(|v| *v == *val).unwrap();

        // Remove it
        let ent = vec.remove(old_pos);

        // Calculate new position
        let new_pos = (old_pos as isize + ent.1 as isize).rem_euclid(vec.len() as isize) as usize;

        // Insert it in to the new position
        vec.insert(new_pos, ent);
    }
}

fn elem(vec: &[(usize, i64)], idx: usize) -> i64 {
    // Get position of the zero
    let pos = vec.iter().position(|(_, v)| *v == 0).unwrap();

    // Return nth element from zero
    vec[(pos + idx) % vec.len()].1
}

// Input parsing

fn input_transform(line: String) -> i64 {
    line.parse::<i64>().expect("Invalid number")
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "1
2
-3
3
-2
0
4
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 3);
        assert_eq!(part2(&input), 1623178306);
    }
}
