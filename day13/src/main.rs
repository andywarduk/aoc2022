use std::cmp::Ordering;
use std::error::Error;

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
    // Read input in chunks of 3 (list, list, empty)
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

// Convert a single u16 to a List containing a single number
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

        // Compare each list item in turn
        for pair in self.items.iter().zip(other.items.iter()) {
            ord = match pair {
                (ListItem::Number(a), ListItem::Number(b)) => (*a).cmp(b),
                (ListItem::Number(a), ListItem::List(b)) => List::from(*a).cmp(b),
                (ListItem::List(a), ListItem::Number(b)) => (*a).cmp(&List::from(*b)),
                (ListItem::List(a), ListItem::List(b)) => (*a).cmp(b),
            };

            if ord != Ordering::Equal {
                // Found a difference
                break;
            }
        }

        if ord == Ordering::Equal {
            // No difference found - lists with fewer items sort first
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
    let mut list = None;

    if !line.is_empty() {
        let mut list_stack: Vec<List> = Vec::new();
        let mut num_start = 0;

        // Flush a number to the current list
        let flush_num = |num_start: &mut usize, next: usize, list_stack: &mut Vec<List>| {
            if *num_start != 0 {
                let index = list_stack.len() - 1;

                list_stack[index].items.push(ListItem::Number(
                    line[*num_start..next]
                        .parse::<u16>()
                        .expect("Should be parseable u16"),
                ));

                *num_start = 0;
            }
        };

        for (i, c) in line.chars().enumerate() {
            match c {
                '[' => {
                    // New list
                    list_stack.push(List::default());
                }
                ']' => {
                    // End of list
                    flush_num(&mut num_start, i, &mut list_stack);

                    if list_stack.len() == 1 {
                        // End of main list - finished
                        list = Some(
                            list_stack
                                .pop()
                                .expect("Should be a list left on the stack"),
                        );

                        break;
                    } else {
                        // End of sub list
                        let sub_list = list_stack.pop().expect("Should be a list on the stack");
                        let index = list_stack.len() - 1;

                        list_stack[index].items.push(ListItem::List(sub_list));
                    }
                }
                '0'..='9' => {
                    // Number
                    if num_start == 0 {
                        num_start = i;
                    }
                }
                ',' => {
                    // Value separator
                    flush_num(&mut num_start, i, &mut list_stack);
                }
                c => panic!("Unexpected character {c} in input"),
            }
        }
    }

    list
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
