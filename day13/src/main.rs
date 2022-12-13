use std::{cmp::Ordering, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(13, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[InputEnt]) -> usize {
    input
        .chunks(3)
        .enumerate()
        .fold(0, |sum, (i, lists)| match lists[0].cmp(&lists[1]) {
            Ordering::Equal => panic!("Equal not expected"),
            Ordering::Greater => sum,
            Ordering::Less => sum + i + 1,
        })
}

fn part2(input: &[InputEnt]) -> usize {
    // Get references to all lists (ignoring separators)
    let mut lists: Vec<&List> = input.iter().filter_map(|ent| ent.as_ref()).collect();

    // Create dividers
    let divider1: List = List {
        items: vec![ListItem::List(List {
            items: vec![ListItem::Number(2)],
        })],
    };

    let divider2: List = List {
        items: vec![ListItem::List(List {
            items: vec![ListItem::Number(6)],
        })],
    };

    // Add dividers to the list
    lists.push(&divider1);
    lists.push(&divider2);

    // Sort the list
    lists.sort();

    // Get positions of the dividers
    let div1pos = lists
        .iter()
        .position(|&x| *x == divider1)
        .expect("Divider 1 should be present")
        + 1;

    let div2pos = lists
        .iter()
        .position(|&x| *x == divider2)
        .expect("Divider 2 should be present")
        + 1;

    // Return result
    div1pos * div2pos
}

#[derive(Debug, Default)]
struct List {
    items: Vec<ListItem>,
}

impl From<u16> for List {
    fn from(value: u16) -> Self {
        List {
            items: vec![ListItem::Number(value)],
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for List {}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ord = Ordering::Equal;

        for pair in self.items.iter().zip(other.items.iter()) {
            ord = match pair {
                (ListItem::Number(a), ListItem::Number(b)) => (*a).cmp(b),
                (ListItem::Number(a), ListItem::List(b)) => List::from(*a).cmp(b),
                (ListItem::List(a), ListItem::Number(b)) => (*a).cmp(&List::from(*b)),
                (ListItem::List(a), ListItem::List(b)) => (*a).cmp(b),
            };

            if ord != Ordering::Equal {
                break;
            }
        }

        if ord == Ordering::Equal {
            ord = self.items.len().cmp(&other.items.len())
        }

        ord
    }
}

#[derive(Debug)]
enum ListItem {
    Number(u16),
    List(List),
}

// Input parsing

type InputEnt = Option<List>;

fn input_transform(line: String) -> InputEnt {
    if line.is_empty() {
        None
    } else {
        let mut list = None;
        let mut list_stack: Vec<List> = Vec::new();
        let mut depth: isize = -1;
        let mut num_start = 0;

        let flush_num =
            |start: &mut usize, next: usize, list_stack: &mut Vec<List>, depth: &mut isize| {
                if *start != 0 {
                    list_stack[*depth as usize].items.push(ListItem::Number(
                        line[*start..next]
                            .parse::<u16>()
                            .expect("Should be parseable u16"),
                    ));
                    *start = 0;
                }
            };

        for (i, c) in line.chars().enumerate() {
            match c {
                '[' => {
                    depth += 1;

                    list_stack.push(List::default());
                }
                ']' => {
                    flush_num(&mut num_start, i, &mut list_stack, &mut depth);

                    depth -= 1;

                    if depth < 0 {
                        list = Some(
                            list_stack
                                .pop()
                                .expect("Should be a list left on the stack"),
                        )
                    } else {
                        let sub_list = list_stack.pop().expect("Should be a list on the stack");

                        list_stack[depth as usize]
                            .items
                            .push(ListItem::List(sub_list));
                    }
                }
                '0'..='9' => {
                    if num_start == 0 {
                        num_start = i;
                    }
                }
                ',' => {
                    flush_num(&mut num_start, i, &mut list_stack, &mut depth);
                }
                c => panic!("Unexpected character {c}"),
            }
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 13);
        assert_eq!(part2(&input), 140);
    }
}
