use crate::board::{Board, Move, Player};

use std::{thread, time};

pub fn make_move(board: &Board) -> Move {
    thread::sleep(time::Duration::from_millis(500));

    let pieces: Vec<(i32, i32)> = board.get_pieces(Player::BLACK);
    let pos = pieces.get(0).unwrap();
    let moves = board.get_legal_moves(pos.clone());

    for pos in pieces.into_iter() {
        let moves = board.get_legal_moves(pos.clone());
        println!("{:?}", moves);
        if moves.len() != 0 {
            return moves.get(0).unwrap().clone();
        }
    }

    return Move::new((0, 0), (0, 0));
}

fn state_value(board: &Board) -> f32 {
    let mut value: f32 = 0.0;

    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board.at((row, col)) {
                match piece.player {
                    Player::RED => value += 1.0,
                    Player::BLACK => value += 1.0
                }
            }
        }
    }

    return value;
}
