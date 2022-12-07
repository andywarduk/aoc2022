use std::{collections::HashMap, error::Error};

use aoc::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(7, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[derive(Debug, Default)]
struct Dir {
    files: Vec<File>,
    dirs: Vec<String>,
}

#[derive(Debug, Default)]
struct File {
    name: String,
    size: usize,
}

fn part1(input: &[InputEnt]) -> usize {
    let tree = build_tree(input);

    let mut total_size = 0;

    for (_, d) in tree.iter() {
        let size = dir_size(&tree, d);

        if size <= 100_000 {
            total_size += size;
        }
    }

    total_size
}

fn part2(input: &[InputEnt]) -> usize {
    let tree = build_tree(input);

    let sizes: Vec<usize> = tree.values().map(|d| dir_size(&tree, d)).collect();

    let total = sizes.iter().max().unwrap();

    let cur_free = 70_000_000 - total;

    let to_free = 30_000_000 - cur_free;

    let free_amount = sizes.iter().filter(|s| **s > to_free).min().unwrap();

    *free_amount
}

fn build_tree(input: &[InputEnt]) -> HashMap<String, Dir> {
    let mut cwd = "".to_string();

    let mut tree: HashMap<String, Dir> = HashMap::new();

    tree.insert(cwd.clone(), Default::default());

    for item in input {
        match item {
            InputEnt::Command(cmd) => {
                let mut split = cmd.split_whitespace();

                match split.next() {
                    Some(exec) => match exec {
                        "cd" => {
                            let to = split.next().unwrap();
                            match to {
                                ".." => match cwd.rfind('/') {
                                    None => cwd = "".to_string(),
                                    Some(p) => cwd.truncate(p),
                                },
                                "/" => cwd = "".to_string(),
                                _ => {
                                    if cwd.is_empty() {
                                        cwd = to.to_string();
                                    } else {
                                        cwd = format!("{cwd}/{to}");
                                    }
                                }
                            }
                        }
                        "ls" => {}
                        _ => panic!("Unrecognised command"),
                    },
                    None => panic!("No command found"),
                }
            }
            InputEnt::Output(line) => {
                let mut split = line.split_whitespace();

                let first = split.next().unwrap();
                match first {
                    "dir" => {
                        let subdir = split.next().unwrap();
                        let path = if cwd.is_empty() {
                            subdir.to_string()
                        } else {
                            format!("{cwd}/{subdir}")
                        };
                        let d = tree.get_mut(&cwd).unwrap();
                        d.dirs.push(path.clone());
                        tree.insert(path, Default::default());
                    }
                    _ => {
                        let size = first.parse::<usize>().unwrap();
                        let file = split.next().unwrap();
                        let d = tree.get_mut(&cwd).unwrap();
                        d.files.push(File {
                            name: file.to_string(),
                            size,
                        })
                    }
                }
            }
        }
    }

    tree
}

fn dir_size(tree: &HashMap<String, Dir>, dir: &Dir) -> usize {
    // Sum files
    let file_sum: usize = dir.files.iter().map(|f| f.size).sum();

    let dir_sum: usize = dir
        .dirs
        .iter()
        .map(|d| {
            let sub_dir = tree.get(d).unwrap();
            dir_size(tree, sub_dir)
        })
        .sum();

    file_sum + dir_sum
}

// Input parsing

enum InputEnt {
    Command(String),
    Output(String),
}

fn input_transform(line: String) -> InputEnt {
    if line.starts_with('$') {
        InputEnt::Command(line[2..].to_string())
    } else {
        InputEnt::Output(line)
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
        assert_eq!(part1(&input), 95437);
        assert_eq!(part2(&input), 24933642);
    }
}
