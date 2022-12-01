use std::error::Error;

use aoc::Input;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = get_input($day)?;

    println!("{:?}", input);

    Ok(())
}

fn get_input(day: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let input = Input::new(day)?;
    let mut result = Vec::new();

    for l in input.lines() {
        let line = l?;

        // TODO process line
        result.push(line);
    }

    Ok(result)
}
