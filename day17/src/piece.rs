use lazy_static::lazy_static;

lazy_static! {
    pub static ref PIECES: Vec<Piece> = vec![
        Piece::new(&[0b1111]),
        Piece::new(&[0b010, 0b111, 0b010]),
        Piece::new(&[0b001, 0b001, 0b111]),
        Piece::new(&[0b1, 0b1, 0b1, 0b1]),
        Piece::new(&[0b11, 0b11]),
    ];
}

pub struct Piece {
    pub bits: Vec<u8>,
    pub width: u8,
    pub height: u8,
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
