pub type InputEnt = Vec<bool>;

pub fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("Unexpected character"),
        })
        .collect()
}
