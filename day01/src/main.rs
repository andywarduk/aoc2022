use std::error::Error;

use aoc::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = get_input(parse_input_vec(1, input_transform)?)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Vec<u64>]) -> u64 {
    input.iter().map(|e| e.iter().sum()).max().unwrap_or(0)
}

fn part2(input: &[Vec<u64>]) -> u64 {
    let mut totals: Vec<u64> = input.iter().map(|e| e.iter().sum()).collect();

    totals.sort_by(|a, b| b.cmp(a));

    totals.iter().take(3).sum()
}

// Input parsing

type InputEnt = u64;

fn get_input(input: Vec<InputEnt>) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    let mut result = Vec::new();

    let mut work_vec = Vec::new();

    let mut proc = |work_vec: &mut Vec<u64>| {
        if !work_vec.is_empty() {
            result.push(work_vec.clone());
            work_vec.clear();
        }
    };

    for cal in input {
        if cal == 0 {
            proc(&mut work_vec);
        } else {
            work_vec.push(cal);
        }
    }

    proc(&mut work_vec);

    Ok(result)
}

fn input_transform(line: String) -> InputEnt {
    line.parse::<InputEnt>().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use aoc::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    #[test]
    fn test1() {
        let input = get_input(parse_test_vec(EXAMPLE1, input_transform).unwrap()).unwrap();
        assert_eq!(part1(&input), 24000);
        assert_eq!(part2(&input), 45000);
    }
}
