#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceKind {
    PAWN,
    KING
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    RED,
    BLACK
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub player: Player
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Move {
    pub from: (i32, i32),
    pub to: (i32, i32)
}

pub struct Board {
    pub pieces: Vec<Option<Piece>>,
    pub player_turn: Player,
    pub selected_piece: (i32, i32),
    pub successive_piece: (i32, i32)
}

impl Piece {
    pub fn new(kind: PieceKind, player: Player) -> Self {
        Self { kind, player }
    }
}

impl Move {
    pub fn new(from: (i32, i32), to: (i32, i32)) -> Self {
        Self { from, to }
    }
}

impl Board {
    pub fn new() -> Self {
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

    pub fn reset(&mut self) {
        *self = Board::new();
    }

    pub fn at(&self, pos: (i32, i32)) -> Option<Piece> {
        if self.in_bounds(pos) {
            return self.pieces[pos.0 as usize * 8 + pos.1 as usize];
        }
        return None;
    }

    pub fn get_pieces(&self, player: Player) -> Vec<(i32, i32)> {
        let mut pieces: Vec<(i32, i32)> = Vec::new();

        for row in 0..8 {
            for col in 0..8 {
                if self.at((row, col)).unwrap().player == player {
                    pieces.push((row, col));
                }
            }
        }

        return pieces;
    }

    pub fn select(&mut self, pos: (i32, i32)) {
        self.selected_piece = pos;
    }

    pub fn deselect(&mut self) {
        self.selected_piece = (-1, -1);
    }

    pub fn is_selected(&self) -> bool {
        return self.selected_piece != (-1, -1);
    }

    pub fn get_selected(&self) -> (i32, i32) {
        return self.selected_piece;
    }

    pub fn set_successive(&mut self, pos: (i32, i32)) {
        self.successive_piece = pos;
    }

    pub fn unset_successive(&mut self) {
        self.successive_piece = (-1, -1);
    }

    pub fn is_successive(&self) -> bool {
        return self.successive_piece != (-1, -1);
    }

    pub fn get_successive(&self)-> (i32, i32) {
        return self.successive_piece;
    }

    pub fn in_bounds(&self, pos: (i32, i32)) -> bool {
        return pos.0 >= 0 && pos.0 <= 7 && pos.1 >= 0 && pos.1 <= 7;
    }

    pub fn is_empty(&self, pos: (i32, i32)) -> bool {
        return self.at(pos).is_none();
    }

    pub fn is_enemy_of(&self, pos: (i32, i32), player: Player) -> bool {
        return self.at(pos).is_some() && self.at(pos).unwrap().player != player;
    }

    pub fn is_kill_available(&self, pos: (i32, i32)) -> bool {
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

    pub fn get_turn(&self) -> Player {
        return self.player_turn;
    }

    pub fn swap_turns(&mut self) {
        if self.player_turn == Player::BLACK {
            self.player_turn = Player::RED
        } else {
            self.player_turn = Player::BLACK;
        }

        self.deselect();
    }

    pub fn is_game_over(&self) -> bool {
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

    pub fn make_king(&mut self, pos: (i32, i32)) {
        let pawn = self.pieces[pos.0 as usize * 8 + pos.1 as usize].unwrap();
        self.pieces[pos.0 as usize * 8 + pos.1 as usize] = Some(Piece::new(PieceKind::KING, pawn.player));
    }

    pub fn is_move_legal_pawn(&self, m: Move, player: Player) -> bool {
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

    pub fn is_move_legal_king(&self, m: Move, player: Player) -> bool {
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

    pub fn is_move_legal(&self, m: Move) -> bool {
        if !self.in_bounds(m.from) || !self.in_bounds(m.to) || self.at(m.from).is_none() || self.at(m.to).is_some() {
            return false;
        }

        let piece = self.at(m.from).unwrap();
        match piece.kind {
            PieceKind::PAWN => return self.is_move_legal_pawn(m, piece.player),
            PieceKind::KING => return self.is_move_legal_king(m, piece.player)
        }
    }

    pub fn is_kill_move(&self, m: &Move, player: Player) -> bool {
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

    // BUG: Down left gives no moves for red
    pub fn get_legal_moves(&self, pos: (i32, i32)) -> Vec<Move> {
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

    pub fn move_piece(&mut self, m: Move) {
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
