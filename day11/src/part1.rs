use std::cmp::Reverse;
use std::collections::VecDeque;

use crate::{InputEnt, Operation};

pub fn part1(input: &[InputEnt]) -> usize {
    let mut monkeys = get_input(input);
    let mut inspections = vec![0; monkeys.len()];

    // Run 20 rounds
    for _ in 0..20 {
        // Each monkey in turn
        (0..monkeys.len()).for_each(|m| loop {
            // Get next item
            let mut worry = match monkeys[m].items.pop_front() {
                None => break,
                Some(n) => n,
            };

            // Increment inspection count
            inspections[m] += 1;

            // Operate on the value
            match monkeys[m].operation {
                Operation::MulOld => worry *= worry,
                Operation::AddOld => worry += worry,
                Operation::MulNum(n) => worry *= n,
                Operation::AddNum(n) => worry += n,
            }

            // Decrease worry to 1/3
            worry /= 3;

            // Work out which monkey to move the item to
            let target = if worry % monkeys[m].test_div == 0 {
                monkeys[m].true_throw
            } else {
                monkeys[m].false_throw
            };

            // Move item to the next monkey
            monkeys[target].items.push_back(worry);
        });
    }

    // Sort inspection counts
    inspections.sort_by_key(|w| Reverse(*w));

    // Return total monkey business
    inspections[0] * inspections[1]
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    monkey: usize,
    items: VecDeque<usize>,
    operation: Operation,
    test_div: usize,
    true_throw: usize,
    false_throw: usize,
}

fn get_input(input: &[InputEnt]) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut monkey = Monkey::default();
    let mut updated = false;

    for ent in input {
        let mut update = true;

        match ent {
            InputEnt::Monkey(n) => monkey.monkey = *n,
            InputEnt::StartItems(items) => monkey.items = items.clone().into(),
            InputEnt::Operation(op) => monkey.operation = op.clone(),
            InputEnt::TestDiv(n) => monkey.test_div = *n,
            InputEnt::Throw(cond, n) => {
                if *cond {
                    monkey.true_throw = *n
                } else {
                    monkey.false_throw = *n
                }
            }
            InputEnt::None => {
                if updated {
                    monkeys.push(monkey);
                }
                monkey = Monkey::default();
                update = false;
            }
        }

        updated = update;
    }

    if updated {
        monkeys.push(monkey);
    }

    monkeys
}
