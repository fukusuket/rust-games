#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Black,
    White,
}

impl Piece {
    pub fn opponent(&self) -> Piece {
        match *self {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }
}

pub const N: Option<Piece> = None;
pub const B: Option<Piece> = Some(Piece::Black);
pub const W: Option<Piece> = Some(Piece::White);