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
    position: (f32, f32)
}

#[derive(Resource)]
struct Board {
    centers: [[(f32, f32); 8]; 8],
    pieces: [[Option<Piece>; 8]; 8]
}

// Used to keep track of a piece being dragged
#[derive(Resource)]
struct DragState {
    initial_position: (f32, f32),
    piece: Option<Piece>
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
            transform: Transform::from_xyz(self.position.0, -self.position.1, 0.0),
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
                if row > 4 && (row + col) as f32 % 2.0 != 0.0 {
                    pieces[row][col] = Some(Piece::new(PieceKind::Red, (x, y)));
                    println!("Generated red piece at {}, {}", x, y);
                }
                if row < 3 && (row + col) as f32 % 2.0 != 0.0 {
                    pieces[row][col] = Some(Piece::new(PieceKind::Black, (x, y)));
                    println!("Generated black piece at {}, {}", x, y);
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

impl DragState {
    fn new() -> DragState {
        DragState {
            initial_position: (-999.99, -999.99),
            piece: None
        }
    }
}

// Making the screen the same size as the board is important for the drag and drop to work
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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(mut commands: Commands, board: ResMut<Board>) {
    board.render(&mut commands);
}

fn input_handler(
    mut commands: Commands,
    mut mouse_button: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    board: ResMut<Board>,
    mut drag: ResMut<DragState>,
)
{
    // If the mouse was just pressed, determine the pressed piece and set it to follow the mouse
    if mouse_button.just_pressed(MouseButton::Left) {
        let mut position: (f32, f32) = (0.0, 0.0);
        if let Some(w_position) = q_windows.single().cursor_position() {
            println!("{}, {}", w_position.x - SCREEN_SIZE / 2.0, w_position.y - SCREEN_SIZE / 2.0);
            position.0 = w_position.x;
            position.1 = w_position.y;

            if let Some(piece) = hit_piece(&mut commands, &board, position) {
                drag.piece = Some(piece);
                drag.initial_position = piece.position;
            }
        }
    }

    // If the mouse is continually pressed down keep the piece following the mouse

    // If the mouse is released and there is a piece being dragged, determine if the new position is valid and place the tile there
    // On mouse release, if a piece is being dragged check if the mouse is in a valid position. Yes -> update piece, No -> Reset position
    if mouse_button.just_released(MouseButton::Left) {

    }
}

fn hit_piece(commands: &mut Commands, board: &ResMut<Board>, mouse_position: (f32, f32)) -> Option<Piece> {
    let center_mouse_position = (mouse_position.0 - SCREEN_SIZE / 2.0, mouse_position.1 - SCREEN_SIZE / 2.0);
    // Check if the mouse position was inside one of the tiles
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board.pieces[row][col] {
                if distance(center_mouse_position, piece.position) <= PIECE_SIZE {
                    println!("Hit a {:?} piece ({}, {}) at {} {}!", piece.kind, piece.position.0, piece.position.1, center_mouse_position.0, center_mouse_position.1);
                    return Some(piece);
                }
            }
        }
    }

    return None;
}

fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((b.0 - a.0) * (b.0 - a.0) + (b.1 - a.1) * (b.1 - a.1)).sqrt()
}
