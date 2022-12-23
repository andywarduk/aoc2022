use crate::{
    map::{Map, Tile},
    Dir, Instruction,
};

pub fn part2(
    input: &[String],
    instructions: &[Instruction],
    cell_width: usize,
    edge_map: &[usize],
) -> usize {
    let map = Map::new(input, cell_width, edge_map);
    let mut state = State::new(&map);

    println!("{:?}", map.edges);

    for i in instructions {
        state.apply(i, &map);
    }

    ((state.y as usize + 1) * 1000) + ((state.x as usize + 1) * 4) + state.dir as usize
}

#[derive(Debug)]
struct State {
    x: isize,
    y: isize,
    dir: Dir,
}

impl State {
    fn new(map: &Map) -> Self {
        let x = map.rows[0].iter().position(|p| *p == Tile::Open).unwrap();

        Self {
            x: x as isize,
            y: 0,
            dir: Dir::Right,
        }
    }

    fn apply(&mut self, instruction: &Instruction, map: &Map) {
        match instruction {
            Instruction::Forward(n) => {
                for _ in 0..*n {
                    let (xadd, yadd) = match self.dir {
                        Dir::Right => (1, 0),
                        Dir::Down => (0, 1),
                        Dir::Left => (-1_isize, 0),
                        Dir::Up => (0, -1_isize),
                    };

                    match map.get_tile(self.x + xadd, self.y + yadd) {
                        Tile::Open => {
                            self.x += xadd;
                            self.y += yadd
                        }
                        Tile::Wall => break,
                        Tile::Void => {
                            let (x, y, dir) = map.cubewrap(self.x, xadd, self.y, yadd);

                            match map.get_tile(x, y) {
                                Tile::Open => {
                                    self.x = x;
                                    self.y = y;
                                    self.dir = dir;
                                }
                                Tile::Wall => break,
                                Tile::Void => panic!("Void -> Void?"),
                            }
                        }
                    }
                    println!("{:?}", self);
                }
            }
            Instruction::Left => {
                self.dir = match self.dir {
                    Dir::Right => Dir::Up,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Down,
                    Dir::Up => Dir::Left,
                };
                println!("{:?}", self);
            }
            Instruction::Right => {
                self.dir = match self.dir {
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                    Dir::Up => Dir::Right,
                };
                println!("{:?}", self);
            }
        }
    }
}
