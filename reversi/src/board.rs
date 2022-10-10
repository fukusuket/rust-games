use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use smallvec::SmallVec;
use crate::coord::Coord;
use crate::piece::{B, N, Piece, W};

const MATRIX_SIZE: usize = 8;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix([[Option<Piece>; MATRIX_SIZE]; MATRIX_SIZE]);

impl Matrix {
    pub fn new() -> Self {
        Matrix([
            [N, N, N, N, N, N, N, N],
            [N, N, N, N, N, N, N, N],
            [N, N, N, N, N, N, N, N],
            [N, N, N, W, B, N, N, N],
            [N, N, N, B, W, N, N, N],
            [N, N, N, N, N, N, N, N],
            [N, N, N, N, N, N, N, N],
            [N, N, N, N, N, N, N, N],
        ])
    }

    pub fn is_in_range(&self, pos: Coord) -> bool {
        let Coord(x, y) = pos;
        0 <= x && x < self.size() as i8 && 0 <= y && y < self.size() as i8
    }

    pub fn size(&self) -> usize {
        MATRIX_SIZE
    }
}

impl Index<Coord> for Matrix {
    type Output = Option<Piece>;

    fn index(&self, index: Coord) -> &Self::Output {
        if !self.is_in_range(index) {
            return &None;
        }
        let Coord(x, y) = index;
        &self.0[y as usize][x as usize]
    }
}

impl IndexMut<Coord> for Matrix {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let Coord(x, y) = index;
        &mut self.0[y as usize][x as usize]
    }
}


impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  1 2 3 4 5 6 7 8")?;
        for y in 0..MATRIX_SIZE {
            write!(f, "{} ", y + 1)?;
            for x in 0..MATRIX_SIZE {
                let cell = self[Coord(x as i8, y as i8)];
                match cell {
                    B => write!(f, "X ")?,
                    W => write!(f, "0 ")?,
                    _ => write!(f, "- ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


const ZERO_FLIP: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub pos: Coord,
    flips: [u8; 8],
}

impl Move {
    pub fn is_legal(&self) -> bool {
        self.flips != ZERO_FLIP
    }
}

pub type Moves<'a> = SmallVec<[Move; MATRIX_SIZE * MATRIX_SIZE]>;

const DIRECTIONS: [Coord; 8] = [
    Coord(-1, -1), //左上
    Coord(0, -1),  //真上
    Coord(1, -1),  //右上
    Coord(-1, 0),  //真左
    Coord(1, 0),   //真右
    Coord(-1, 1),  //左下
    Coord(0, 1),   //真下
    Coord(1, 1),   //右下
];

#[derive(Clone, Debug)]
pub struct Board {
    pub matrix: Matrix,
    pub black: u8,
    pub white: u8,
}

impl Board {
    pub fn new() -> Self {
        Board {
            matrix: Matrix::new(),
            black: 2,
            white: 2,
        }
    }

    fn get_flip(&self, piece: Piece, mut pos: Coord, dir: Coord) -> u8 {
        let me = Some(piece);
        let mut count = 0;
        loop {
            pos += dir;
            let cell = self.matrix[pos];
            if cell == N {
                return 0;
            }
            if cell == me {
                return count;
            }
            count += 1;
        }
    }

    fn get_move(&self, piece: Piece, pos: Coord) -> Move {
        if self.matrix[pos] != N {
            return Move {
                pos,
                flips: ZERO_FLIP,
            };
        }
        let mut flips = [0; 8];
        for (dir, flip) in DIRECTIONS.iter().zip(flips.iter_mut()) {
            *flip = self.get_flip(piece, pos, *dir);
        }
        Move { pos, flips }
    }

    pub fn moves(&self, piece:Piece) -> Moves {
        let mut moves = SmallVec::new();
        for y in 0..self.matrix.size(){
            for x in 0..self.matrix.size() {
                let mov = self.get_move(piece, Coord(x as i8, y as i8));
                if mov.is_legal(){
                    moves.push(mov);
                }
            }
        }
        moves
    }

    fn count_mut(&mut self, piece: Piece) -> &mut u8 {
        match piece {
            Piece::Black => &mut self.black,
            Piece::White => &mut self.white,
        }
    }

    fn do_flip(&mut self, piece: Piece, mut pos: Coord, dir: Coord, flip: u8){
        for _ in 0..flip {
            pos += dir;
            self.matrix[pos] = Some(piece);
        }

        *self.count_mut(piece) += flip;
        *self.count_mut(piece.opponent()) -= flip;
    }

    pub fn do_move(&mut self, piece: Piece, mov: &Move) {
        self.matrix[mov.pos] = Some(piece);
        for (flip, dir) in mov.flips.into_iter().zip(DIRECTIONS.iter()) {
            self.do_flip(piece, mov.pos, *dir, flip);
        }
        *self.count_mut(piece) += 1;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.matrix)
    }
}