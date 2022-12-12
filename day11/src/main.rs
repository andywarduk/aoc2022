use std::error::Error;

use aoc::input::parse_input_vec;

use part1::part1;
use part2::part2;

mod part1;
mod part2;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(11, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[derive(Debug, Default, Clone)]
pub enum Operation {
    #[default]
    AddOld,
    MulOld,
    AddNum(usize),
    MulNum(usize),
}

// Input parsing

pub enum InputEnt {
    Monkey(usize),
    StartItems(Vec<usize>),
    Operation(Operation),
    TestDiv(usize),
    Throw(bool, usize),
    None,
}

fn input_transform(line: String) -> InputEnt {
    let mut terms = line.split_whitespace();

    match terms.next() {
        None => InputEnt::None,
        Some("Monkey") => InputEnt::Monkey(
            terms
                .next()
                .unwrap()
                .trim_end_matches(':')
                .parse::<usize>()
                .unwrap(),
        ),
        Some("Starting") => InputEnt::StartItems(
            terms
                .skip(1)
                .map(|t| t.trim_end_matches(',').parse().unwrap())
                .collect(),
        ),
        Some("Operation:") => {
            assert_eq!(terms.next(), Some("new"));
            assert_eq!(terms.next(), Some("="));
            assert_eq!(terms.next(), Some("old"));

            InputEnt::Operation(match (terms.next(), terms.next()) {
                (Some("*"), Some("old")) => Operation::MulOld,
                (Some("+"), Some("old")) => Operation::AddOld,
                (Some("*"), Some(num)) => Operation::MulNum(num.parse::<usize>().unwrap()),
                (Some("+"), Some(num)) => Operation::AddNum(num.parse::<usize>().unwrap()),
                _ => panic!("Unknown operator"),
            })
        }
        Some("Test:") => {
            assert_eq!(terms.next(), Some("divisible"));
            assert_eq!(terms.next(), Some("by"));

            InputEnt::TestDiv(terms.next().unwrap().parse::<usize>().unwrap())
        }
        Some("If") => {
            let if_bool = match terms.next() {
                Some("true:") => true,
                Some("false:") => false,
                _ => panic!("Unknown if condition"),
            };

            assert_eq!(terms.next(), Some("throw"));
            assert_eq!(terms.next(), Some("to"));
            assert_eq!(terms.next(), Some("monkey"));

            InputEnt::Throw(if_bool, terms.next().unwrap().parse::<usize>().unwrap())
        }
        _ => panic!("Unexpected term"),
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 10605);
        assert_eq!(part2(&input), 2713310158);
    }
}
