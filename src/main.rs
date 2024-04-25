use bevy::prelude::*;

const SCREEN_SIZE: f32 = 800.0;
const TILE_SIZE: f32 = SCREEN_SIZE / 8.0;
const PIECE_SIZE: f32 = TILE_SIZE / 2.0;

#[derive(Clone, Copy)]
enum PieceKind {
    Red,
    Black,
    RedKing,
    BlackKing
}

#[derive(Component, Clone, Copy)]
struct Piece {
    kind: PieceKind,
    position: (f32, f32)
}

#[derive(Resource)]
struct Board {
    centers: [[(f32, f32); 8]; 8],
    pieces: [[Option<Piece>; 8]; 8]
}

impl Piece {
    fn new(kind: PieceKind, position: (f32, f32)) -> Piece {
        Piece {
            kind,
            position
        }
    }

    fn render(&self, commands: &mut Commands) {
        let color;
        match self.kind {
            PieceKind::Red |
            PieceKind::RedKing => color = Color::RED,
            PieceKind::Black |
            PieceKind::BlackKing => color = Color::INDIGO
        }
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(PIECE_SIZE, PIECE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(self.position.0, self.position.1, 0.0),
            ..default()
        });
    }
}

impl Board {
    fn new() -> Board {
        let mut centers = [[(0.0, 0.0); 8]; 8];
        let mut pieces = [[None; 8]; 8];

        for row in 0..8 {
            for col in 0..8 {
                // Generate the center positions of all tiles for rendering later
                let x = TILE_SIZE / 2.0 - SCREEN_SIZE / 2.0 + TILE_SIZE * col as f32;
                let y = TILE_SIZE / 2.0 - SCREEN_SIZE / 2.0 + TILE_SIZE * row as f32;
                centers[row][col] = (x, y);

                // Generate the pieces. These have independent positions for drag and drop to make sense
                if row > 4 && (row + col) as f32 % 2.0 == 0.0 {
                    pieces[row][col] = Some(Piece::new(PieceKind::Red, (x, y)));
                }
                if row < 3 && (row + col) as f32 % 2.0 == 0.0 {
                    pieces[row][col] = Some(Piece::new(PieceKind::Black, (x, y)));
                }
            }
        }

        Board {
            centers,
            pieces
        }
    }

    fn render(&self, commands: &mut Commands) {
        for row in 0..8 {
            for col in 0..8 {
                let color;
                if (row + col) as f32 % 2.0 == 0.0 {
                    color = Color::BLACK;
                } else {
                    color = Color::WHITE;
                }

                // This renders one tile
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(self.centers[row][col].0, self.centers[row][col].1, 0.0),
                    ..default()
                });

                // This renders a piece on the tile
                if let Some(piece) = self.pieces[row][col] {
                    piece.render(commands);
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .insert_resource(Board::new())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(mut commands: Commands, board: ResMut<Board>) {
    board.render(&mut commands);
}
