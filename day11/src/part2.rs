use std::cmp::Reverse;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::{InputEnt, Operation};

pub fn part2(input: &[InputEnt]) -> usize {
    // Destructure input
    let mut monkeys = get_input(input);

    let mut inspections = vec![0; monkeys.len()];

    // Run 10,000 rounds
    for _ in 0..10_000 {
        // Each monkey in turn
        (0..monkeys.len()).for_each(|m| loop {
            // Get next item
            let mut worry = match monkeys[m].items.pop_front() {
                None => break,
                Some(n) => n,
            };

            // Increment inspection count
            inspections[m] += 1;

            // Apply operation to the worry
            match &monkeys[m].operation {
                Operation::MulOld => worry.square(),
                Operation::AddOld => worry.double(),
                Operation::MulNum(n) => worry.mul(*n),
                Operation::AddNum(n) => worry.add(*n),
            };

            // Work out which monkey to move the item to
            let target = if worry.remainders[m] == 0 {
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

#[derive(Debug, Clone)]
struct Worry {
    remainders: Vec<usize>,
    divisors: Rc<Vec<usize>>,
}

impl Worry {
    fn new(value: usize, divisors: Rc<Vec<usize>>) -> Self {
        let remainders = divisors.iter().map(|d| value % *d).collect();

        Self {
            remainders,
            divisors,
        }
    }

    fn square(&mut self) {
        for i in 0..self.remainders.len() {
            self.remainders[i] = (self.remainders[i] * self.remainders[i]) % self.divisors[i];
        }
    }

    fn double(&mut self) {
        for i in 0..self.remainders.len() {
            self.remainders[i] = (self.remainders[i] + self.remainders[i]) % self.divisors[i];
        }
    }

    fn add(&mut self, value: usize) {
        for i in 0..self.remainders.len() {
            self.remainders[i] = (self.remainders[i] + value) % self.divisors[i];
        }
    }

    fn mul(&mut self, value: usize) {
        for i in 0..self.remainders.len() {
            self.remainders[i] = (self.remainders[i] * value) % self.divisors[i];
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    monkey: usize,
    items: VecDeque<Worry>,
    operation: Operation,
    true_throw: usize,
    false_throw: usize,
}

fn get_input(input: &[InputEnt]) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut monkey = Monkey::default();
    let mut updated = false;

    // Build dividers vector
    let divisors = Rc::new(
        input
            .iter()
            .filter_map(|i| match i {
                InputEnt::TestDiv(n) => Some(*n),
                _ => None,
            })
            .collect::<Vec<_>>(),
    );

    for ent in input {
        let mut update = true;

        match ent {
            InputEnt::Monkey(n) => monkey.monkey = *n,
            InputEnt::StartItems(items) => {
                monkey.items = items
                    .iter()
                    .map(|i| Worry::new(*i, divisors.clone()))
                    .collect::<VecDeque<Worry>>()
            }
            InputEnt::Operation(op) => monkey.operation = op.clone(),
            InputEnt::TestDiv(_) => (),
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
