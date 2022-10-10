use std::ops::{Add, AddAssign};

/// ベクトルを表現する構造体
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord(pub i8, pub i8);

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Coord(x1, y1) = self;
        let Coord(x2, y2) = rhs;
        Coord(x1 + x2, y1 + y2)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        let &mut Coord(ref mut x1, ref mut y1) = self;
        let Coord(x2, y2) = rhs;
        *x1 += x2;
        *y1 += y2;
    }
}