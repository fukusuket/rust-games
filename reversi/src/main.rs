use crate::board::{Board};
use crate::game::Game;
use crate::piece::Piece;
use crate::player::Human;

mod board;
mod piece;
mod game;
mod coord;
mod player;


fn main() {
    let p1 = Human;
    let p2 = Human;
    let mut game = Game::new(p1, p2);
    loop {
        game.print();
        game.step();
    }
}