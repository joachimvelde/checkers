mod board;
mod bot;

use board::*;
use bot::*;

use raylib::prelude::*;

const PIECE_RADIUS: f32 = 30.0;

fn mark_tile(d: &mut RaylibDrawHandle, width: &i32, height: &i32, row: i32, col: i32) {
    let tile_width = width / 8;
    let tile_height = height / 8;

    let rect = Rectangle::new(col as f32 * tile_width as f32, row as f32 * tile_height as f32, tile_width as f32, tile_height as f32);
    d.draw_rectangle_lines_ex(rect, 7.5, Color::LIME);
}

fn draw_tiles(d: &mut RaylibDrawHandle, board: &Board, width: &i32, height: &i32) {
    let tile_width = width / 8;
    let tile_height = height / 8;

    // Draw the black and white tiles
    for row in 0..8 {
        for col in 0..8 {
            let colour = if (row + col) % 2 == 0 { Color::WHITE } else { Color::BLACK };
            d.draw_rectangle(col * tile_width, row * tile_height, tile_width, tile_height, colour);
        }
    }

    // Mark the correct tiles
    for row in 0..8 {
        for col in 0..8 {
            if board.is_selected() && board.get_selected() == (row, col) {
                mark_tile(d, width, height, row, col);

                for m in board.get_legal_moves(board.get_selected()) {
                    mark_tile(d, width, height, m.to.0, m.to.1);
                }
            }
        }
    }
}

fn draw_pieces(d: &mut RaylibDrawHandle, board: &Board, width: &i32, height: &i32) {
    let tile_width = width / 8;
    let tile_height = height / 8;

    for row in 0..8 {
        for col in 0..8 {
            match board.at((row, col)) {
                Some(piece) => {
                    let x = tile_width / 2 + col as i32 * tile_width;
                    let y = tile_height / 2 + row as i32 * tile_height;

                    match (piece.kind, piece.player) {
                        (PieceKind::PAWN, Player::RED) => d.draw_circle(x, y, PIECE_RADIUS, Color::RED),
                        (PieceKind::KING, Player::RED) => {
                            d.draw_circle(x, y, PIECE_RADIUS, Color::RED);
                            // d.draw_circle_lines(x, y, PIECE_RADIUS, Color::GOLD);
                            d.draw_circle(x, y, PIECE_RADIUS / 5.0, Color::GOLD);
                        },
                        (PieceKind::PAWN, Player::BLACK) => d.draw_circle(x, y, PIECE_RADIUS, Color::GRAY),
                        (PieceKind::KING, Player::BLACK) => {
                            d.draw_circle(x, y, PIECE_RADIUS, Color::GRAY);
                            // d.draw_circle_lines(x, y, PIECE_RADIUS, Color::GOLD);
                            d.draw_circle(x, y, PIECE_RADIUS / 5.0, Color::GOLD);
                        }
                    }
                },
                None => ()
            }
        }
    }
}

fn draw(mut d: RaylibDrawHandle, board: &Board, width: &i32, height: &i32) {
    draw_tiles(&mut d, board, width, height);
    draw_pieces(&mut d, board, width, height);

    let rect = Rectangle::new(0.0, 0.0, *width as f32, *height as f32);
    let colour = if board.player_turn == Player::RED { Color::RED } else { Color::BLACK };
    d.draw_rectangle_lines_ex(rect, 3.0, colour);
}

fn update(rl: &mut RaylibHandle, board: &mut Board, mouse: &Vector2) {
    let (row, col) = ((mouse.y / 100.0).floor() as i32, (mouse.x / 100.0).floor() as i32);

    if board.get_turn() == Player::BLACK {
        let m = get_bot_move(&board);
        board.move_piece(m);
    } else if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
        if board.at((row, col)).is_some() && board.at((row, col)).unwrap().player == board.get_turn() {
            board.select((row, col));
        } else if board.is_selected() && board.at(board.get_selected()).unwrap().player == board.get_turn() {
            let m = Move::new(board.get_selected(), (row, col));
            if board.get_legal_moves(board.get_selected()).contains(&m) {
                board.deselect();
                board.move_piece(m);
            }
        }
    }

    if board.is_game_over() {
        board.reset();
    }
}

fn main() {
    let width: i32 = 800;
    let height: i32 = 800;

    let mut board = Board::new();

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Checkers")
        .build();

    let mut d = rl.begin_drawing(&thread);
    draw(d, &board, &width, &height);
    while !rl.window_should_close() {
        let mouse: Vector2 = rl.get_mouse_position();
        update(&mut rl, &mut board, &mouse);

        let mut d = rl.begin_drawing(&thread);
        draw(d, &board, &width, &height);
    }
}
