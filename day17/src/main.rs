use std::error::Error;

use lazy_static::lazy_static;

use aoc::input::parse_input_line;

lazy_static! {
    static ref PIECES: Vec<Piece> = vec![
        Piece::new(&[0b1111]),
        Piece::new(&[0b010, 0b111, 0b010]),
        Piece::new(&[0b001, 0b001, 0b111]),
        Piece::new(&[0b1, 0b1, 0b1, 0b1]),
        Piece::new(&[0b11, 0b11]),
    ];
}

struct Piece {
    bits: Vec<u8>,
    width: u8,
    height: u8,
}

impl Piece {
    fn new(bits: &[u8]) -> Self {
        let width = bits
            .iter()
            .map(|r| (8 - r.leading_zeros()) as u8)
            .max()
            .unwrap();

        Self {
            bits: Vec::from(bits),
            width,
            height: bits.len() as u8,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(17, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &InputEnt) -> usize {
    let mut board = Vec::new();

    let mut piece_iter = PIECES.iter().cycle();
    let mut move_iter = input.iter().cycle();

    let test_move = |board: &Vec<_>, piece: &Piece, pos, shift| -> bool {
        let mut hit = false;

        for i in (0..piece.height).rev() {
            let row = pos - i as usize;

            if row < board.len() {
                // Check for clash
                if board[row] & (piece.bits[i as usize] << shift) != 0 {
                    hit = true;
                    break;
                }
            }
        }

        hit
    };

    for _ in 0..2022 {
        // Next piece
        let piece = piece_iter.next().unwrap();
        let mut piece_pos = board.len() + 2 + piece.height as usize;
        let piece_shift_max = 7 - piece.width;
        let mut piece_shift = piece_shift_max - 2;

        // println!(
        //     "pos={piece_pos}, shift={piece_shift}, height={}",
        //     piece.height
        // );

        loop {
            // Move left/right
            let new_piece_shift = match move_iter.next().unwrap() {
                Dir::Left => {
                    if piece_shift < piece_shift_max {
                        piece_shift + 1
                    } else {
                        piece_shift
                    }
                }
                Dir::Right => piece_shift.saturating_sub(1),
            };

            if new_piece_shift != piece_shift
                && !test_move(&board, piece, piece_pos, new_piece_shift)
            {
                piece_shift = new_piece_shift;
            }

            // println!("shift={piece_shift}");

            // Move down
            if piece_pos == piece.height as usize - 1
                || test_move(&board, piece, piece_pos - 1, piece_shift)
            {
                break;
            }

            piece_pos -= 1;

            // println!("pos={piece_pos}");
        }

        // println!("rest pos={piece_pos}");
        for i in (0..piece.height as usize).rev() {
            let row = piece_pos - i;
            // println!("row {row}");
            let bits = piece.bits[i] << piece_shift;

            if row >= board.len() {
                // println!("add {bits:b}");
                board.push(bits);
            } else {
                // println!("or {bits:b}");
                board[row] |= bits;
            }
        }

        // TODO remove
        // for i in board.iter().rev() {
        //     let line: String = format!("|{:07b}|", i)
        //         .chars()
        //         .map(|c| match c {
        //             '0' => ' ',
        //             '1' => '#',
        //             c => c,
        //         })
        //         .collect();
        //     println!("{line}");
        // }
        // println!("----");
    }

    board.len()
}

fn part2(input: &InputEnt) -> u64 {
    0 // TODO
}

enum Dir {
    Left,
    Right,
}

// Input parsing

type InputEnt = Vec<Dir>; // TODO

fn input_transform(line: String) -> InputEnt {
    line.chars()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            _ => panic!("Unexpected char {c}"),
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test1() {
        let input = input_transform(EXAMPLE1.to_string());
        assert_eq!(part1(&input), 3068);
        assert_eq!(part2(&input), 1514285714288);
    }
}
