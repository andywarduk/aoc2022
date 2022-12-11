use std::{collections::VecDeque, error::Error};

use aoc::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = get_input(parse_input_vec(11, input_transform)?);

    dbg!(&input);
    // Run parts
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));

    Ok(())
}

fn part1(mut monkeys: Vec<Monkey>) -> usize {
    for _ in 0..20 {
        (0..monkeys.len()).for_each(|m| loop {
            let mut worry = match monkeys[m].items.pop_front() {
                None => break,
                Some(n) => n,
            };

            monkeys[m].inspections += 1;

            let op_val = match monkeys[m].op_val {
                OpVal::Old => worry,
                OpVal::Num(n) => n,
            };

            match monkeys[m].operator {
                Operator::Mul => worry *= op_val,
                Operator::Add => worry += op_val,
            }

            worry /= 3;

            let target = if worry % monkeys[m].test_div == 0 {
                monkeys[m].true_throw
            } else {
                monkeys[m].false_throw
            };

            monkeys[target].items.push_back(worry);
        });

        // for m in &monkeys {
        //     println!("{} ({}): {:?}", m.monkey, m.inspections, m.items);
        // }
    }

    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();

    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

fn part2(monkeys: Vec<Monkey>) -> usize {
    0 // TODO
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    monkey: usize,
    items: VecDeque<usize>,
    operator: Operator,
    op_val: OpVal,
    test_div: usize,
    true_throw: usize,
    false_throw: usize,
    inspections: usize,
}

#[derive(Debug, Default, Clone)]
enum Operator {
    #[default]
    Add,
    Mul,
}

#[derive(Debug, Default, Clone)]
enum OpVal {
    #[default]
    Old,
    Num(usize),
}

// Input parsing

fn get_input(input: Vec<InputEnt>) -> Vec<Monkey> {
    let mut rules = Vec::new();
    let mut rule = Monkey::default();
    let mut updated = false;

    for ent in input {
        let mut update = true;

        match ent {
            InputEnt::Monkey(n) => rule.monkey = n,
            InputEnt::StartItems(items) => rule.items = items.into(),
            InputEnt::Operation(op, op_val) => {
                rule.operator = op;
                rule.op_val = op_val
            }
            InputEnt::TestDiv(n) => rule.test_div = n,
            InputEnt::Throw(cond, n) => {
                if cond {
                    rule.true_throw = n
                } else {
                    rule.false_throw = n
                }
            }
            InputEnt::None => {
                if updated {
                    rules.push(rule);
                }
                rule = Monkey::default();
                update = false;
            }
        }

        updated = update;
    }

    if updated {
        rules.push(rule);
    }

    rules
}

enum InputEnt {
    Monkey(usize),
    StartItems(Vec<usize>),
    Operation(Operator, OpVal),
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

            let op = match terms.next() {
                Some("*") => Operator::Mul,
                Some("+") => Operator::Add,
                _ => panic!("Unknown operator"),
            };

            let val = match terms.next() {
                Some("old") => OpVal::Old,
                Some(num) => OpVal::Num(num.parse::<usize>().unwrap()),
                _ => panic!("Unknown operator value"),
            };

            InputEnt::Operation(op, val)
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
    use aoc::parse_test_vec;

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
        let input = get_input(parse_test_vec(EXAMPLE1, input_transform).unwrap());
        assert_eq!(part1(input.clone()), 10605);
        assert_eq!(part2(input), 0 /* TODO */);
    }
}
