use crate::board::{Board, Move, Player};

use std::{thread, time};

pub fn make_move(board: &Board) -> Move {
    thread::sleep(time::Duration::from_millis(10));

    let pieces: Vec<(i32, i32)> = board.get_pieces(Player::BLACK);
    let moves = board.get_legal_moves(pieces.get(0));
    return moves.get(0);
}
