use std::{collections::HashMap, error::Error};

use aoc::parse_input_vec;

const TOTAL_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(7, input_transform)?;

    // Build file system tree
    let tree = build_tree(&input);

    // Run parts
    println!("Part 1: {}", part1(&tree));
    println!("Part 2: {}", part2(&tree));

    Ok(())
}

#[derive(Debug, Default)]
struct Dir {
    size: usize,
}

fn part1(tree: &HashMap<String, Dir>) -> usize {
    let mut total_size = 0;

    for (_, d) in tree.iter() {
        if d.size <= 100_000 {
            total_size += d.size;
        }
    }

    total_size
}

fn part2(tree: &HashMap<String, Dir>) -> usize {
    let total = tree.get("").unwrap().size;

    let cur_free = TOTAL_SPACE - total;

    let to_free = NEEDED_SPACE - cur_free;

    let free_amount = tree
        .values()
        .filter_map(|d| if d.size > to_free { Some(d.size) } else { None })
        .min()
        .unwrap();

    free_amount
}

fn build_tree(input: &[InputEnt]) -> HashMap<String, Dir> {
    let mut cwd = "".to_string();

    let mut tree: HashMap<String, Dir> = HashMap::new();
    tree.insert(cwd.clone(), Default::default());

    for item in input {
        match item {
            InputEnt::CommandCdRoot => cwd = "".to_string(),
            InputEnt::CommandCdUp => cwd.truncate(cwd.rfind('/').unwrap_or(0)),
            InputEnt::CommandCd(to) => {
                if cwd.is_empty() {
                    cwd = to.to_string();
                } else {
                    cwd = format!("{cwd}/{to}");
                }
            }
            InputEnt::CommandLs => {}
            InputEnt::OutputDir(dir) => {
                let path = if cwd.is_empty() {
                    dir.to_string()
                } else {
                    format!("{cwd}/{dir}")
                };

                tree.insert(path, Default::default());
            }
            InputEnt::OutputFile(size) => {
                let mut dir = cwd.clone();

                loop {
                    let d = tree.get_mut(&dir).unwrap();
                    d.size += size;

                    if dir.is_empty() {
                        break;
                    }

                    dir.truncate(dir.rfind('/').unwrap_or(0));
                }
            }
        }
    }

    tree
}

// Input parsing

enum InputEnt {
    CommandCdRoot,
    CommandCdUp,
    CommandCd(String),
    CommandLs,
    OutputDir(String),
    OutputFile(usize),
}

fn input_transform(line: String) -> InputEnt {
    let mut split = line.split_whitespace();

    match split.next().unwrap() {
        "$" => match split.next().unwrap() {
            "cd" => match split.next().unwrap() {
                ".." => InputEnt::CommandCdUp,
                "/" => InputEnt::CommandCdRoot,
                x => InputEnt::CommandCd(x.to_string()),
            },
            "ls" => InputEnt::CommandLs,
            x => panic!("Unknown command {x}"),
        },
        "dir" => InputEnt::OutputDir(split.next().unwrap().to_string()),
        x => InputEnt::OutputFile(x.parse::<usize>().unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use aoc::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test1() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        let tree = build_tree(&input);
        assert_eq!(part1(&tree), 95437);
        assert_eq!(part2(&tree), 24933642);
    }
}
