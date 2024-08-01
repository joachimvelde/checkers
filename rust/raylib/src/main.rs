use raylib::prelude::*;

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

#[derive(Clone, Copy, PartialEq, Debug)]
struct Move {
    from: (i32, i32),
    to: (i32, i32)
}

struct Board {
    pieces: Vec<Option<Piece>>,
    player_turn: Player,
    selected_piece: (i32, i32),
    successive_piece: (i32, i32)
}

impl Piece {
    fn new(kind: PieceKind, player: Player) -> Self {
        Self { kind, player }
    }
}

impl Move {
    fn new(from: (i32, i32), to: (i32, i32)) -> Self {
        Self { from, to }
    }
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
            selected_piece: (-1, -1),
            successive_piece: (-1, -1)
        }
    }

    fn reset(&mut self) {
        *self = Board::new();
    }

    fn at(&self, pos: (i32, i32)) -> Option<Piece> {
        if self.in_bounds(pos) {
            return self.pieces[pos.0 as usize * 8 + pos.1 as usize];
        }
        return None;
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

    fn set_successive(&mut self, pos: (i32, i32)) {
        self.successive_piece = pos;
    }

    fn unset_successive(&mut self) {
        self.successive_piece = (-1, -1);
    }

    fn is_successive(&self) -> bool {
        return self.successive_piece != (-1, -1);
    }

    fn get_successive(&self)-> (i32, i32) {
        return self.successive_piece;
    }

    fn in_bounds(&self, pos: (i32, i32)) -> bool {
        return pos.0 >= 0 && pos.0 <= 7 && pos.1 >= 0 && pos.1 <= 7;
    }

    fn is_empty(&self, pos: (i32, i32)) -> bool {
        return self.at(pos).is_none();
    }

    fn is_enemy_of(&self, pos: (i32, i32), player: Player) -> bool {
        return self.at(pos).is_some() && self.at(pos).unwrap().player != player;
    }

    fn is_kill_available(&self, pos: (i32, i32)) -> bool {
        if self.at(pos).is_none() {
            return true;
        }

        let piece = self.at(pos).unwrap();
        let player = piece.player;

        let kill_moves = match piece.kind {
            PieceKind::PAWN => match player {
                Player::RED => vec![
                    (pos.0 - 2, pos.1 - 2, pos.0 - 1, pos.1 - 1),
                    (pos.0 - 2, pos.1 + 2, pos.0 - 1, pos.1 + 1)
                ],
                Player::BLACK => vec![
                    (pos.0 + 2, pos.1 - 2, pos.0 + 1, pos.1 - 1),
                    (pos.0 + 2, pos.1 + 2, pos.0 + 1, pos.1 + 1)
                ]
            },
            PieceKind::KING => vec![
                (pos.0 - 2, pos.1 - 2, pos.0 - 1, pos.1 - 1),
                (pos.0 - 2, pos.1 + 2, pos.0 - 1, pos.1 + 1),
                (pos.0 + 2, pos.1 - 2, pos.0 + 1, pos.1 - 1),
                (pos.0 + 2, pos.1 + 2, pos.0 + 1, pos.1 + 1)
            ]
        };

        for &(x_to, y_to, x_mid, y_mid) in &kill_moves {
            if self.in_bounds((x_to, y_to)) && self.is_empty((x_to, y_to)) && self.is_enemy_of((x_mid, y_mid), player) {
                return true;
            }
        }

        return false;
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

    fn is_game_over(&self) -> bool {
        let mut reds = 0;
        let mut blacks = 0;

        for row in 0..8 {
            for col in 0..8 {
                let piece = self.at((row, col));
                if piece.is_some() {
                    match piece.unwrap().player {
                        Player::RED => reds += 1,
                        Player::BLACK => blacks += 1
                    }
                }
            }
        }

        return reds == 0 || blacks == 0;
    }

    fn make_king(&mut self, pos: (i32, i32)) {
        let pawn = self.pieces[pos.0 as usize * 8 + pos.1 as usize].unwrap();
        self.pieces[pos.0 as usize * 8 + pos.1 as usize] = Some(Piece::new(PieceKind::KING, pawn.player));
    }

    fn is_move_legal_pawn(&self, m: Move, player: Player) -> bool {
        if !self.in_bounds(m.from) || !self.in_bounds(m.to) || self.at(m.from).is_none() || self.at(m.to).is_some() {
            return false;
        }

        let from = m.from;
        let to = m.to;
        let dir = match player {
            Player::RED => -1,
            Player::BLACK => 1
        };

        // Normal moves
        if to == (from.0 + dir, from.1 - 1) || to == (from.0 + dir, from.1 + 1) {
            return true;
        }

        let kill_moves = vec![
            (from.0 + 2 * dir, from.1 - 2, from.0 + dir, from.1 - 1),
            (from.0 + 2 * dir, from.1 + 2, from.0 + dir, from.1 + 1)
        ];

        for &(x_to, y_to, x_mid, y_mid) in &kill_moves {
            if to == (x_to, y_to) && self.at((x_mid, y_mid)).map_or(false, |piece| piece.player != player) {
                return true;
            }
        }

        return false;
    }

    fn is_move_legal_king(&self, m: Move, player: Player) -> bool {
        if !self.in_bounds(m.from) || !self.in_bounds(m.to) || self.at(m.from).is_none() || self.at(m.to).is_some() {
            return false;
        }

        let from = m.from;
        let to = m.to;

        // Normal moves
        if (to.0 == from.0 - 1 || to.0 == from.0 + 1) && (to.1 == from.1 - 1 || to.1 == from.1 + 1) {
            return true;
        }

        let kill_moves = vec![
            (from.0 - 2, from.1 - 2, from.0 - 1, from.1 - 1),
            (from.0 - 2, from.1 + 2, from.0 - 1, from.1 + 1),
            (from.0 + 2, from.1 - 2, from.0 + 1, from.1 - 1),
            (from.0 + 2, from.1 + 2, from.0 + 1, from.1 + 1)
        ];

        for &(x_to, y_to, x_mid, y_mid) in &kill_moves {
            if to == (x_to, y_to) && self.at((x_mid, y_mid)).map_or(false, |piece| piece.player != player) {
                return true;
            }
        }

        return false;
    }

    fn is_move_legal(&self, m: Move) -> bool {
        if !self.in_bounds(m.from) || !self.in_bounds(m.to) || self.at(m.from).is_none() || self.at(m.to).is_some() {
            return false;
        }

        let piece = self.at(m.from).unwrap();
        match piece.kind {
            PieceKind::PAWN => return self.is_move_legal_pawn(m, piece.player),
            PieceKind::KING => return self.is_move_legal_king(m, piece.player)
        }
    }

    fn is_kill_move(&self, m: &Move, player: Player) -> bool {
        if !self.in_bounds(m.to) {
            return false;
        }

        let dx = (m.to.0 - m.from.0).abs();
        let dy = (m.to.1 - m.from.1).abs();

        if dx == 2 && dy == 2 && self.is_empty(m.to) {
            let mx = (m.from.0 + m.to.0) / 2;
            let my = (m.from.1 + m.to.1) / 2;


            if let Some(piece) = self.at((mx, my)) {
                return piece.player != player;
            }
        }

        return false;
    }

    fn get_legal_moves(&self, pos: (i32, i32)) -> Vec<Move> {
        assert!(self.at(pos).is_some());

        let piece = self.at(pos).unwrap();

        let moves = vec![
            Move::new(pos, (pos.0 + 1, pos.1 + 1)),
            Move::new(pos, (pos.0 + 1, pos.1 - 1)),
            Move::new(pos, (pos.0 - 1, pos.1 + 1)),
            Move::new(pos, (pos.0 - 1, pos.1 - 1)),
            Move::new(pos, (pos.0 + 2, pos.1 + 2)),
            Move::new(pos, (pos.0 + 2, pos.1 - 2)),
            Move::new(pos, (pos.0 - 2, pos.1 + 2)),
            Move::new(pos, (pos.0 - 2, pos.1 - 2))
        ];

        let (kill_moves, normal_moves): (Vec<Move>, Vec<Move>) = moves
            .into_iter()
            .partition(|m| self.is_kill_move(m, piece.player));

        // If there are killing moves, the player cannot execute non-kill moves
        if kill_moves.is_empty() {
            return normal_moves.into_iter().filter(|m| self.is_move_legal(*m)).collect();
        } else {
            return kill_moves.into_iter().filter(|m| self.is_move_legal(*m)).collect();
        }
    }

    fn move_piece(&mut self, m: Move) {
        // Move piece
        self.pieces[m.to.0 as usize * 8 + m.to.1 as usize] = self.pieces[m.from.0 as usize * 8 + m.from.1 as usize];
        self.pieces[m.from.0 as usize * 8 + m.from.1 as usize] = None;

        // Execute kill
        if (m.to.0 - m.from.0).abs() == 2 {
            let middle: (i32, i32) = (
                (m.from.0 + m.to.0) / 2,
                (m.from.1 + m.to.1) / 2
            );
            self.pieces[middle.0 as usize * 8 + middle.1 as usize] = None;

            // Multi-kill moves
            if self.is_kill_available(m.to) {
                self.set_successive(m.to);
            } else {
                self.unset_successive();
                self.swap_turns();
            }
        } else {
            self.unset_successive();
            self.swap_turns();
        }

        let piece = self.at(m.to).unwrap();
        match (m.to.0, piece.kind, piece.player) {
            (0, PieceKind::PAWN, Player::RED) | (7, PieceKind::PAWN, Player::BLACK) => self.make_king(m.to),
            _default => ()
        }
    }
}

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

    if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
        if board.at((row, col)).is_some() && board.at((row, col)).unwrap().player == board.get_turn() {
            board.select((row, col));
        } else if board.is_selected() && board.at(board.get_selected()).unwrap().player == board.player_turn {
            let m = Move::new(board.get_selected(), (row, col));
            if board.get_legal_moves(board.get_selected()).contains(&m) {
                board.deselect();
                board.move_piece(m);
            }
        }

        if board.is_game_over() {
            board.reset();
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
