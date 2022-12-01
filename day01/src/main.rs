use std::error::Error;

use aocinput::Input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = get_input(1)?;

    part1(&input);
    part2(&input);

    Ok(())
}

fn part1(input: &Vec<u64>) {
    let mut tot = 0;
    let mut max = 0;

    let mut proc = |tot: &mut u64| {
        if *tot > max {
            max = *tot;
        }

        *tot = 0;
    };

    for cal in input {
        if *cal == 0 {
            proc(&mut tot);
        } else {
            tot += cal;
        }
    }

    proc(&mut tot);

    println!("Part 1: {}", max);
}

fn part2(input: &Vec<u64>) {
    let mut tot = 0;
    let mut totals: Vec<u64> = Vec::new();

    let mut proc = |tot: &mut u64| {
        totals.push(*tot);
        *tot = 0;
    };

    for cal in input {
        if *cal == 0 {
            proc(&mut tot);
        } else {
            tot += cal;
        }
    }

    proc(&mut tot);

    totals.sort_by(|a, b| b.cmp(a));

    let ans: u64 = totals.iter().take(3).sum();

    println!("Part 2: {:?}", ans);
}

fn get_input(day: usize) -> Result<Vec<u64>, Box<dyn Error>> {
    let input = Input::new(day)?;
    let mut result = Vec::new();

    for l in input.lines() {
        let line = l?;

        result.push(line.parse::<u64>().unwrap_or(0));
    }

    Ok(result)
}
