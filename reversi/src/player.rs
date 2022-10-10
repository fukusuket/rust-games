use std::io;
use crate::{Board, Piece};
use crate::board::Move;
use crate::coord::Coord;
use crate::game::Play;

pub struct Human;
impl Play for Human {
    fn play(&mut self, piece: Piece, board: &Board) -> Option<Move> {
        let moves = board.moves(piece);
        if moves.len() == 0 {
            return None;
        }
        loop {
            println!("こまをおく、たてのかずをおしえてください");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let y: Result<usize, _> = input.trim().parse();
            let y = y.unwrap() as i8;

            println!("こまをおく、よこのかずをおしえてください");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let x: Result<usize, _> = input.trim().parse();
            let x = x.unwrap() as i8;

            let c = Coord{ 0: x - 1, 1: y - 1 };
            for k in moves.iter(){
                if k.pos == c {
                    return Some(k.clone())
                }
            }
            println!("そこには、こまをおけません。")
        }
    }
}