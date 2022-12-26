use std::error::Error;

use aoc::input::parse_input_vec;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_vec(25, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));

    Ok(())
}

fn part1(input: &[isize]) -> String {
    let total: isize = input.iter().sum();

    build_snafu(total)
}

fn parse_snafu(string: &str) -> isize {
    string
        .chars()
        .map(|c| match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Invalid digit {c}"),
        })
        .rev()
        .enumerate()
        .fold(0, |acc, (i, d)| acc + (5_isize.pow(i as u32) * d))
}

fn check_remainder(rem: isize, pow: isize) -> bool {
    if pow > 1 && rem != 0 {
        let divisor = pow / 5;
        let digit = (rem / divisor).abs();

        match digit {
            2 => check_remainder(rem % divisor, divisor),
            _ if digit < 2 => true,
            _ => false,
        }
    } else {
        true
    }
}

fn build_snafu(mut num: isize) -> String {
    let mut pow = 1;
    let mut loops = 1;

    loop {
        if num / pow <= 2 {
            break;
        }

        pow *= 5;
        loops += 1;
    }

    let mut result = String::new();

    for _ in 0..loops {
        let mut digit = num / pow;
        let rem = num % pow;

        if !check_remainder(rem, pow) {
            if rem > 0 {
                digit += 1
            } else {
                digit -= 1
            }
        }

        match digit {
            2 => result.push('2'),
            1 => result.push('1'),
            0 => result.push('0'),
            -1 => result.push('-'),
            -2 => result.push('='),
            _ => unreachable!(),
        }

        num -= digit * pow;
        pow /= 5;
    }

    result
}

// Input parsing

fn input_transform(line: String) -> isize {
    parse_snafu(&line)
}

#[cfg(test)]
mod tests {
    use aoc::input::parse_test_vec;

    use super::*;

    const EXAMPLE1: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    fn test1() {
        assert_eq!(build_snafu(1), "1");
        assert_eq!(build_snafu(2), "2");
        assert_eq!(build_snafu(3), "1=");
        assert_eq!(build_snafu(4), "1-");
        assert_eq!(build_snafu(5), "10");
        assert_eq!(build_snafu(6), "11");
        assert_eq!(build_snafu(7), "12");
        assert_eq!(build_snafu(8), "2=");
        assert_eq!(build_snafu(9), "2-");
        assert_eq!(build_snafu(10), "20");
        assert_eq!(build_snafu(15), "1=0");
        assert_eq!(build_snafu(20), "1-0");
        assert_eq!(build_snafu(2022), "1=11-2");
        assert_eq!(build_snafu(12345), "1-0---0");
        assert_eq!(build_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test2() {
        let input = parse_test_vec(EXAMPLE1, input_transform).unwrap();
        assert_eq!(
            input,
            vec![1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37]
        );
        assert_eq!(part1(&input), "2=-1=0");
    }
}
