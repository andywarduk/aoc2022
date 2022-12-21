use std::{collections::HashMap, error::Error};

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(21, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[Monkey]) -> EqnNum {
    // Build hashmap from input
    let eqns = input
        .iter()
        .map(|i| (i.name.clone(), i))
        .collect::<HashMap<_, _>>();

    // Compute
    calc_monkey(&eqns, "root")
}

fn calc_monkey(monkeys: &HashMap<String, &Monkey>, elem: &str) -> EqnNum {
    match &monkeys[elem].act {
        Action::Number(n) => *n,
        Action::Eqn(a, op, b) => {
            let anum = calc_monkey(monkeys, a);
            let bnum = calc_monkey(monkeys, b);

            match op {
                Op::Add => anum + bnum,
                Op::Sub => anum - bnum,
                Op::Div => anum / bnum,
                Op::Mul => anum * bnum,
            }
        }
    }
}

fn part2(input: &[Monkey]) -> EqnNum {
    let (mut ansmap, mut eqns, roots) = input.iter().fold(
        (HashMap::new(), Vec::new(), Vec::new()),
        |(mut ansmap, mut eqns, mut roots), m| {
            match m.name.as_str() {
                "root" => match &m.act {
                    Action::Eqn(a, _, b) => {
                        roots = vec![a.clone(), b.clone()];
                    }
                    _ => unreachable!(),
                },
                "humn" => (),
                _ => match m.act {
                    Action::Number(n) => {
                        ansmap.insert(m.name.clone(), n);
                    }
                    Action::Eqn(_, _, _) => eqns.push(m),
                },
            }
            (ansmap, eqns, roots)
        },
    );

    loop {
        let old_len = eqns.len();

        eqns.retain(|e| match &e.act {
            Action::Eqn(a, op, b) => match (ansmap.get(a), ansmap.get(b)) {
                (Some(anum), Some(bnum)) => {
                    let res = match op {
                        Op::Add => anum + bnum,
                        Op::Sub => anum - bnum,
                        Op::Div => anum / bnum,
                        Op::Mul => anum * bnum,
                    };

                    ansmap.insert(e.name.clone(), res);

                    false
                }
                _ => true,
            },
            _ => unreachable!(),
        });

        if old_len == eqns.len() {
            break;
        }
    }

    // Get target
    let target = match (ansmap.get(&roots[0]), ansmap.get(&roots[1])) {
        (Some(t), None) => *t,
        (None, Some(t)) => *t,
        _ => unreachable!(),
    };

    // Build tree of remaining equations
    let termval = |t: &str| -> TermVal {
        match ansmap.get(t) {
            Some(n) => TermVal::Number(*n),
            _ => match t {
                "humn" => TermVal::Unknown,
                _ => TermVal::Unresolved(t.to_string()),
            },
        }
    };

    let mut terms = eqns
        .into_iter()
        .map(|e| match &e.act {
            Action::Eqn(a, op, b) => {
                let left = termval(a);
                let right = termval(b);

                Term {
                    name: e.name.clone(),
                    left,
                    op: op.clone(),
                    right,
                    reference: None,
                }
            }
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    // Resolve unknowns
    let resolve = |terms: &mut Vec<Term>, t: &str, i| -> TermVal {
        let pos = find_term(terms, t);

        terms[pos].reference = Some(i);

        TermVal::Term(pos)
    };

    for i in 0..terms.len() {
        if let TermVal::Unresolved(t) = terms[i].left.clone() {
            terms[i].left = resolve(&mut terms, &t, i);
        }
        if let TermVal::Unresolved(t) = terms[i].right.clone() {
            terms[i].right = resolve(&mut terms, &t, i);
        }
    }

    // Find root equation
    let unk = terms
        .iter()
        .position(|t| t.left == TermVal::Unknown || t.right == TermVal::Unknown)
        .expect("Unknown value not found");

    let mut root = unk;

    while let Some(parent) = terms[root].reference {
        root = parent
    }

    // Back calculate
    match back_calc(&terms, root, target) {
        TermVal::Number(n) => n,
        _ => unreachable!(),
    }
}

fn find_term(terms: &[Term], t: &str) -> usize {
    terms
        .iter()
        .position(|term| term.name == *t)
        .expect("Term not found")
}

fn back_calc(terms: &Vec<Term>, idx: usize, acc: EqnNum) -> TermVal {
    match (terms[idx].left.clone(), terms[idx].right.clone()) {
        (TermVal::Number(l), TermVal::Number(r)) => TermVal::Number(match terms[idx].op {
            Op::Add => l + r,
            Op::Sub => l - r,
            Op::Div => l / r,
            Op::Mul => l * r,
        }),
        (TermVal::Number(l), r) => {
            // n op term
            let new_acc = match terms[idx].op {
                Op::Add => acc - l, // l + x = acc -> x = acc - l
                Op::Sub => l - acc, // l - x = acc -> x = l - acc
                Op::Div => l / acc, // l / x = acc => x = l / acc
                Op::Mul => acc / l, // l * x = acc => x = acc / l
            };

            match r {
                TermVal::Term(r) => back_calc(terms, r, new_acc),
                TermVal::Unknown => TermVal::Number(new_acc),
                _ => unreachable!(),
            }
        }
        (l, TermVal::Number(r)) => {
            // term op n
            let new_acc = match terms[idx].op {
                Op::Add => acc - r, // x - r = acc -> x = acc - r
                Op::Sub => acc + r, // x + r = acc -> x = acc - r
                Op::Div => acc * r, // x * r = acc -> x = acc / r
                Op::Mul => acc / r, // x / r = acc -> x = acc * r
            };

            match l {
                TermVal::Term(l) => back_calc(terms, l, new_acc),
                TermVal::Unknown => TermVal::Number(new_acc),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Term {
    name: String,
    left: TermVal,
    op: Op,
    right: TermVal,
    reference: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TermVal {
    Number(EqnNum),
    Unknown,
    Term(usize),
    Unresolved(String),
}

// Input parsing

type EqnNum = u64;

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    act: Action,
}

#[derive(Debug, Clone)]
enum Action {
    Number(EqnNum),
    Eqn(String, Op, String),
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

fn input_transform(line: String) -> Monkey {
    let mut iter = line.split(':');

    let name = iter.next().expect("Name not found");

    let actstr = iter
        .next()
        .expect("Action term not found")
        .trim()
        .split(' ')
        .collect::<Vec<_>>();

    let act = match actstr.len() {
        1 => Action::Number(actstr[0].parse::<EqnNum>().expect("Invalid number")),
        3 => {
            let op = match actstr[1] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                _ => panic!("Invalid op"),
            };
            Action::Eqn(actstr[0].to_string(), op, actstr[2].to_string())
        }
        _ => panic!("Invalid monkey spec"),
    };

    Monkey {
        name: name.to_string(),
        act,
    }
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(part1(&input), 152);
        assert_eq!(part2(&input), 301);
    }
}
