use std::{collections::HashMap, error::Error};

use aoc::input::parse_input_line;
use piece::{Piece, PIECES};

mod piece;

fn main() -> Result<(), Box<dyn Error>> {
    // Get input
    let input = parse_input_line(17, input_transform)?;

    // Run parts
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &InputEnt) -> usize {
    stack_pieces(input, 2022, |_, _, _, _| {}).len()
}

fn part2(input: &InputEnt) -> usize {
    let mut rep_height = 0;
    let mut hashmap = HashMap::new();
    let mut scanning = true;

    stack_pieces(input, 1_000_000_000_000, |board, iter, mvno, pieceno| {
        if scanning {
            // Calculate profile
            let mut mask = 0;
            let mut profile = Vec::new();

            for i in (0..board.len()).rev() {
                mask |= board[i];

                if mask == 0x7f {
                    break;
                }

                profile.push(mask);
            }

            // Create hashmap entry
            let entry = MapEntry {
                mvno,
                pieceno,
                profile,
            };

            // Already exists?
            if let Some((entry, height)) = hashmap.get(&entry) {
                // Yes - found repeat
                let repeat_size = *iter - entry;
                let repeat_cnt = (1_000_000_000_000usize - *iter) / repeat_size;

                rep_height = (board.len() - height) * repeat_cnt;

                *iter += repeat_cnt * repeat_size;

                scanning = false;
            } else {
                // No - insert it
                hashmap.insert(entry, (*iter, board.len()));
            }
        }
    })
    .len()
        + rep_height
}

fn stack_pieces<F>(input: &InputEnt, iterations: usize, mut cb: F) -> Vec<u8>
where
    F: FnMut(&Vec<u8>, &mut usize, usize, usize),
{
    let mut board = Vec::new();

    let mut piece_iter = PIECES.iter().enumerate().cycle();
    let mut move_iter = input.iter().enumerate().cycle();

    let mut iter = 0;

    loop {
        // Next piece
        let (pieceno, piece) = piece_iter.next().unwrap();
        let mut piece_pos = board.len() + 2 + piece.height as usize;
        let piece_shift_max = 7 - piece.width;
        let mut piece_shift = piece_shift_max - 2;
        let mut first_mvno = None;

        loop {
            // Move left/right
            let (mvno, mv) = move_iter.next().unwrap();

            if first_mvno.is_none() {
                first_mvno = Some(mvno);
            }

            let new_piece_shift = match mv {
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

            // Move down
            if piece_pos == piece.height as usize - 1
                || test_move(&board, piece, piece_pos - 1, piece_shift)
            {
                break;
            }

            piece_pos -= 1;
        }

        // Put the piece on the board
        for i in (0..piece.height as usize).rev() {
            let row = piece_pos - i;
            let bits = piece.bits[i] << piece_shift;

            if row >= board.len() {
                board.push(bits);
            } else {
                board[row] |= bits;
            }
        }

        // Call callback
        cb(&board, &mut iter, first_mvno.unwrap(), pieceno);

        // Finished?
        iter += 1;

        if iter >= iterations {
            break;
        }
    }

    board
}

fn test_move(board: &Vec<u8>, piece: &Piece, pos: usize, shift: u8) -> bool {
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
}

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash)]
struct MapEntry {
    mvno: usize,
    pieceno: usize,
    profile: Vec<u8>,
}

// Input parsing

type InputEnt = Vec<Dir>;

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
