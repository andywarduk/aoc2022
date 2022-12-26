use crate::dir::Dir;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InTile {
    Wall,
    Empty,
    Blizzard(Dir),
}

pub type InputEnt = Vec<InTile>;

pub fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '#' => InTile::Wall,
            '.' => InTile::Empty,
            '>' => InTile::Blizzard(Dir::Right),
            '<' => InTile::Blizzard(Dir::Left),
            '^' => InTile::Blizzard(Dir::Up),
            'v' => InTile::Blizzard(Dir::Down),
            _ => panic!("Invalid char {line}"),
        })
        .collect()
}
