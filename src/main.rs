use bevy::{
    prelude::*,
    input::mouse::MouseButton,
    window::{PrimaryWindow, WindowResolution}
};

const SCREEN_SIZE: f32 = 800.0;
const TILE_SIZE: f32 = SCREEN_SIZE / 8.0;
const PIECE_SIZE: f32 = TILE_SIZE / 2.0;

#[derive(Clone, Copy, Debug)]
enum PieceKind {
    Red,
    Black,
    RedKing,
    BlackKing
}

#[derive(Component, Clone, Copy)]
struct Piece {
    kind: PieceKind,
    position: Vec2
}

#[derive(Resource)]
struct Board {
    centers: [[Vec2; 8]; 8],
    pieces: [[Option<Piece>; 8]; 8]
}

#[derive(Resource)]
struct DragState {
    initial_position: Vec2,
    piece: Option<Piece>
}

impl Piece {
    fn new(kind: PieceKind, position: Vec2) -> Self {
        Self {
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
            transform: Transform::from_xyz(self.position.x, self.position.y, 0.0),
            ..default()
        });
    }
}

impl Board {
    fn new() -> Self {
        let mut centers = [[Vec2::default(); 8]; 8];
        let mut pieces = [[None; 8]; 8];

        for row in 0..8 {
            for col in 0..8 {
                let x = -SCREEN_SIZE / 2.0 + TILE_SIZE / 2.0 + TILE_SIZE * col as f32;
                let y = -SCREEN_SIZE / 2.0 + TILE_SIZE / 2.0 + TILE_SIZE * (7 - row) as f32;
                centers[row][col].x = x;
                centers[row][col].y = y;

                if row < 3 && (row + col) as f32 % 2.0 != 0.0 {
                    pieces[row][col] = Some(Piece::new(PieceKind::Red, Vec2::new(x, y)));
                    eprintln!("Red: {:?}", Vec2::new(x, y));
                }

                if row > 4 && (row + col) as f32 % 2.0 != 0.0 {
                    pieces[row][col] = Some(Piece::new(PieceKind::Black, Vec2::new(x, y)));
                    eprintln!("Black: {:?}", Vec2::new(x, y));
                }
            }
        }

        Board { centers, pieces }
    }

    fn render(&self, commands: &mut Commands) {
        for row in 0..8 {
            for col in 0..8 {
                let color;
                if (row + col) as f32 % 2.0 == 0.0 {
                    color = Color::WHITE;
                } else {
                    color = Color::BLACK;
                }

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(self.centers[row][col].x, self.centers[row][col].y, 0.0),
                    ..default()
                });

                if let Some(piece) = self.pieces[row][col] {
                    piece.render(commands);
                }
            }
        }
    }
}

impl DragState {
    fn new() -> DragState {
        DragState {
            initial_position: Vec2::default(),
            piece: None
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Checkers".to_string(),
                resolution: WindowResolution::new(SCREEN_SIZE, SCREEN_SIZE),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, input_handler))
        .insert_resource(Board::new())
        .insert_resource(DragState::new())
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(mut commands: Commands, board: ResMut<Board>) {
    board.render(&mut commands);
}

fn input_handler(
    mut commands: Commands,
    mut left_mouse: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    board: ResMut<Board>,
    mut drag: ResMut<DragState>,
)
{
    if left_mouse.just_pressed(MouseButton::Left) {
        if let Some(w_position) = q_windows.single().cursor_position() {
            if let Some(piece) = mouse_hit(&mut commands, &board, w_position) {
                drag.piece = Some(piece);
                drag.initial_position = piece.position; // NOTE: Maybe clone?
            }
        }
    }
}

// NOTE: We have to center the mouse coordinates
fn mouse_hit(commands: &mut Commands, board: &ResMut<Board>, mouse_position: Vec2) -> Option<Piece> {
    let centered = Vec2::new(mouse_position.x - SCREEN_SIZE / 2.0,
                             -mouse_position.y + SCREEN_SIZE / 2.0);

    eprintln!("Centered mouse: {:?}", centered);
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board.pieces[row][col] {
                if distance(centered, piece.position) <= PIECE_SIZE {
                    eprintln!("Hit a {:?} piece at {:?}", piece.kind, piece.position);
                    return Some(piece);
                }
            }
        }
    }

    None
}

fn distance(a: Vec2, b: Vec2) -> f32 {
    ((b.x - a.x) * (b.x - a.x) + (b.y - a.y) * (b.y - a.y)).sqrt()
}
