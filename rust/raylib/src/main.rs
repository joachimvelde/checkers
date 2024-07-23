use raylib::prelude::*;

/*
Create board so that is is independant on raylib. Create functions that communicate with raylib to call methods on the
board to update it's state and make moves. For example; draw should not be part of Board, but rather take board as an
argument and draw it.
*/

const PIECE_RADIUS: f32 = 30.0;

#[derive(Clone, Copy, PartialEq, Debug)]
enum PieceKind {
    PAWN,
    KING
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Player {
    RED,
    BLACK
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Piece {
    kind: PieceKind,
    player: Player
}

impl Piece {
    fn new(kind: PieceKind, player: Player) -> Self {
        Self { kind, player }
    }
}

struct Board {
    pieces: Vec<Option<Piece>>,
    player_turn: Player,
    selected_piece: (i32, i32)
}

impl Board {
    fn new() -> Self {
        let mut pieces: Vec<Option<Piece>> = vec![None; 8 * 8];

        // Create the black pieces
        for row in 0..3 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    pieces[row * 8 + col] = Some(Piece::new(PieceKind::PAWN, Player::BLACK));
                }
            }
        }

        // Create the red pieces
        for row in 5..8 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    pieces[row * 8 + col] = Some(Piece::new(PieceKind::PAWN, Player::RED));
                }
            }
        }

        Self {
            pieces: pieces,
            player_turn: Player::BLACK,
            selected_piece: (-1, -1)
        }
    }

    // TODO: Use this
    fn at(&self, pos: (i32, i32)) -> Option<Piece> {
        return self.pieces[pos.0 as usize * 8 + pos.1 as usize];
    }

    fn select(&mut self, pos: (i32, i32)) {
        self.selected_piece = pos;
    }

    fn deselect(&mut self) {
        self.selected_piece = (-1, -1);
    }

    fn is_selected(&self) -> bool {
        return self.selected_piece != (-1, -1);
    }

    fn get_selected(&self) -> (i32, i32) {
        return self.selected_piece;
    }

    fn get_turn(&self) -> Player {
        return self.player_turn;
    }

    fn swap_turns(&mut self) {
        if self.player_turn == Player::BLACK {
            self.player_turn = Player::RED
        } else {
            self.player_turn = Player::BLACK;
        }

        self.deselect();
    }

    fn is_move_legal(&self, from: (i32, i32), to: (i32, i32)) -> bool {
        true
    }

    fn move_piece(&mut self, from: (i32, i32), to: (i32, i32)) {
        if !self.is_move_legal(from, to) { return };
    
        self.pieces[to.0 as usize* 8 + to.1 as usize] = self.pieces[from.0 as usize * 8 + from.1 as usize];
        self.pieces[from.0 as usize * 8 + from.1 as usize] = None;
    }
}

fn mark_tile(d: &mut RaylibDrawHandle, width: &i32, height: &i32, row: i32, col: i32) {
    let tile_width = width / 8;
    let tile_height = height / 8;

    let rect = Rectangle::new(col as f32* tile_width as f32, row as f32* tile_height as f32, tile_width as f32, tile_height as f32);
    d.draw_rectangle_lines_ex(rect, 7.5, Color::LIME);
}

fn draw_tiles(d: &mut RaylibDrawHandle, board: &Board, width: &i32, height: &i32) {
    let tile_width = width / 8;
    let tile_height = height / 8;

    for row in 0..8 {
        for col in 0..8 {
            let colour = if (row + col) % 2 == 0 { Color::WHITE } else { Color::BLACK };
            d.draw_rectangle(col * tile_width, row * tile_height, tile_width, tile_height, colour);

            if board.is_selected() && board.get_selected() == (row, col) {
                mark_tile(d, width, height, row, col);
            }
        }
    }
}

fn draw_pieces(d: &mut RaylibDrawHandle, board: &Board, width: &i32, height: &i32) {
    let tile_width = width / 8;
    let tile_height = height / 8;

    for row in 0..8 {
        for col in 0..8 {
            match board.pieces[row * 8 + col] {
                Some(piece) => {
                    let x = tile_width / 2 + col as i32 * tile_width;
                    let y = tile_height / 2 + row as i32 * tile_height;

                    match (piece.kind, piece.player) {
                        (PieceKind::PAWN, Player::RED) => d.draw_circle(x, y, PIECE_RADIUS, Color::RED),
                        (PieceKind::KING, Player::RED) => println!("Draw a red king!"),
                        (PieceKind::PAWN, Player::BLACK) => d.draw_circle(x, y, PIECE_RADIUS, Color::GRAY),
                        (PieceKind::KING, Player::BLACK) => println!("Draw a black king!")
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
}

fn update(rl: &mut RaylibHandle, board: &mut Board, mouse: &Vector2) {
    let (row, col) = ((mouse.y / 100.0).floor() as i32, (mouse.x / 100.0).floor() as i32);

    if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
        if !board.at((row, col)).is_none() && board.at((row, col)).unwrap().player == board.get_turn() {
            board.select((row, col));
        } else if board.is_selected() && board.at(board.get_selected()).unwrap().player == board.player_turn {
            board.move_piece(board.get_selected(), (row, col));
            board.swap_turns();
        }
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

    while !rl.window_should_close() {
        let mouse: Vector2 = rl.get_mouse_position();
        update(&mut rl, &mut board, &mouse);

        let mut d = rl.begin_drawing(&thread);
        draw(d, &board, &width, &height);
    }
}
