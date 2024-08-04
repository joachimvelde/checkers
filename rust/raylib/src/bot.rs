use crate::board::{Board, Move, Player, PieceKind};

use std::{thread, time};

const MINIMAX_DEPTH: i32 = 7;

pub fn get_bot_move(board: &Board) -> Move {
    // thread::sleep(time::Duration::from_millis(500));
    let (_, m) = minimax(board, MINIMAX_DEPTH, -f32::INFINITY, f32::INFINITY);
    return m.unwrap();
}

pub fn minimax(board: &Board, depth: i32, alpha: f32, beta: f32) -> (f32, Option<Move>) {
    if board.is_game_over() || depth == 0 {
        return (state_value(board), None);
    }

    let mut alpha = alpha;
    let mut beta = beta;
    let mut best_move = Move::default();

    if board.get_turn() == Player::BLACK {
        let mut value = f32::INFINITY;
        for m in board.get_all_legal_moves(Player::BLACK) {
            let (next_value, _) = minimax(&result(&board, m), depth - 1, alpha, beta);

            if next_value < value {
                best_move = m;
                value = next_value;
            }

            beta = beta.min(value);
            if beta <= alpha {
                break;
            }
        }
        return (value, Some(best_move));
    } else {
        let mut value = -f32::INFINITY;
        for m in board.get_all_legal_moves(Player::RED) {
            let (next_value, _) = minimax(&result(&board, m), depth - 1, alpha, beta);

            if next_value > value {
                best_move = m;
                value = next_value;
            }

            alpha = alpha.max(value);
            if alpha >= alpha {
                break;
            }
        }
        return (value, Some(best_move));
    }
}

fn result(board: &Board, m: Move) -> Board {
    let mut new_board = board.clone();
    new_board.move_piece(m);
    return new_board;
}

pub fn state_value(board: &Board) -> f32 {
    let mut value: f32 = 0.0;

    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board.at((row, col)) {
                match (piece.player, piece.kind) {
                    (Player::RED, PieceKind::PAWN) => value += 1.0,
                    (Player::BLACK, PieceKind::PAWN) => value -= 1.0,
                    (Player::RED, PieceKind::KING) => value += 2.0,
                    (Player::BLACK, PieceKind::KING) => value -= 2.0,
                }
            }
        }
    }

    return value;
}
