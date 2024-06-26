use bevy::{
    prelude::*,
    input::mouse::MouseButton,
    window::{PrimaryWindow, WindowResolution}
};

const SCREEN_SIZE: f32 = 800.0;
const TILE_SIZE: f32 = SCREEN_SIZE / 8.0;
const PIECE_SIZE: f32 = TILE_SIZE / 2.0;

#[derive(Debug, Copy, PartialEq, Clone)]
enum PieceKind {
    Regular,
    King
}

#[derive(Debug, Copy, PartialEq, Clone)]
enum Player {
    Red,
    Black
}

#[derive(PartialEq)]
enum TileKind {
    Black,
    White
}

#[derive(Component, Copy, Clone, Debug)]
struct Piece {
    kind: PieceKind,
    owner: Player
}

#[derive(Component, Copy, Clone, PartialEq, Debug)]
struct Position {
    row: i32,
    col: i32
}

#[derive(Component)]
struct Tile {
    kind: TileKind
}

#[derive(Resource)]
struct DragState {
    piece: Option<Entity>,
    initial_position: Vec2,
    offset: Vec2
}

#[derive(Resource)]
struct TurnState {
    kind: Player
}

impl DragState {
    fn new() -> Self {
        Self {
            piece: None,
            initial_position: Vec2::default(),
            offset: Vec2::default()
        }
    }
}

impl TurnState {
    fn new() -> Self {
        Self {
            kind: Player::Red // Red starts
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Checkers".to_string(),
                resolution: WindowResolution::new(SCREEN_SIZE, SCREEN_SIZE),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .insert_resource(DragState::new())
        .insert_resource(TurnState::new())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for row in 0..8 {
        for col in 0..8 {
            let x = -SCREEN_SIZE / 2.0 + TILE_SIZE / 2.0 + TILE_SIZE * col as f32;
            let y = -SCREEN_SIZE / 2.0 + TILE_SIZE / 2.0 + TILE_SIZE * (7 - row) as f32;

            // Create tiles
            let mut color = Color::BLACK;
            let mut kind = TileKind::Black;
            if (row + col) as f32 % 2.0 == 0.0 {
                color = Color::WHITE;
                kind = TileKind::White;
            }
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert(Tile { kind })
            .insert(Position{ row, col });

            // Create pieces
            if row < 3 && (row + col) as f32 % 2.0 != 0.0 {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(PIECE_SIZE, PIECE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 1.0),
                    ..default()
                })
                .insert(Piece {
                    kind: PieceKind::Regular,
                    owner: Player::Red
                })
                .insert(Position { row, col });
            }

            if row > 4 && (row + col) as f32 % 2.0 != 0.0 {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::PURPLE,
                        custom_size: Some(Vec2::new(PIECE_SIZE, PIECE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 1.0),
                    ..default()
                })
                .insert(Piece {
                    kind: PieceKind::Regular,
                    owner: Player::Black
                })
                .insert(Position { row, col });
            }
        }
    }
}

fn update
(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    left_mouse: Res<ButtonInput<MouseButton>>,
    tiles: Query<(Entity, &Tile, &Position, &Transform), Without<Piece>>,
    mut pieces: Query<(Entity, &Piece, &mut Position, &mut Transform), Without<Tile>>,
    mut drag: ResMut<DragState>,
    mut turn: ResMut<TurnState>
)
{
    let pieces_vec: Vec<(Piece, Position)> = pieces.iter().map(|(_, piece, position, _)| return (*piece, *position)).collect();

    let (camera, camera_transform) = q_camera.single();
    if let Some(mouse_position) = window.single().cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
            {
                // Check for hits
                if left_mouse.just_pressed(MouseButton::Left) {
                    for (entity, piece, position, transform) in pieces.iter() {
                        // Detect hits
                        let center = transform.translation.truncate();
                        if mouse_position.distance(center) <= PIECE_SIZE / 2.0 && piece.owner == turn.kind {
                            drag.piece = Some(entity);
                            drag.initial_position.x = transform.translation.x;
                            drag.initial_position.y = transform.translation.y;
                            break;
                        }
                    }
                }

                if let Some(entity) = drag.piece {
                    if let Ok((_, _, _, mut transform)) = pieces.get_mut(entity) {
                        transform.translation.x = mouse_position.x;
                        transform.translation.y = mouse_position.y;
                        transform.translation.z = 2.0;
                    }
                }

                // Check for drops
                let mut kill_position = Position { row: -1, col: -1 };
                if left_mouse.just_released(MouseButton::Left) {
                    if let Some(entity) = drag.piece {
                        drag.piece = None;
                        if let Ok((_, piece, mut piece_position, mut piece_transform)) = pieces.get_mut(entity) {
                            piece_transform.translation.z = 1.0;
                            // Place at closest tile
                            let mut placed = false;
                            for (_entity, tile, tile_position, tile_transform) in tiles.iter() {
                                let tile_center = tile_transform.translation.truncate();
                                if mouse_position.distance(tile_center) <= TILE_SIZE / 2.0 && tile.kind == TileKind::Black {
                                    let old_pos: Position = *piece_position;
                                    let new_pos: Position = *tile_position;

                                    // Check if the position is occupied
                                    let (position_valid, kill_pos_temp) = position_is_valid(&pieces_vec, &piece, old_pos, new_pos);
                                    if position_valid {
                                        piece_transform.translation.x = tile_transform.translation.x;
                                        piece_transform.translation.y = tile_transform.translation.y;

                                        // Update position of piece
                                        piece_position.row = tile_position.row;
                                        piece_position.col = tile_position.col;
                                        placed = true;

                                        // Update player turn
                                        turn.kind = if piece.owner == Player::Black { Player::Red } else { Player::Black };

                                        kill_position = kill_pos_temp;

                                        break;
                                    } else {
                                        println!("Attempted to move to an invalid position");
                                    }
                                }
                            }
                            // Reset position
                            if !placed {
                                piece_transform.translation.x = drag.initial_position.x;
                                piece_transform.translation.y = drag.initial_position.y;
                            }
                        }
                        // Despawn killed piece
                        if kill_position.row != -1 {
                            for (entity, _, position, _) in pieces.iter() {
                                if *position == kill_position {
                                    commands.entity(entity).despawn();
                                }
                            }
                        }
                    }
                }
            }
}

fn position_is_valid(pieces_vec: &Vec<(Piece, Position)>, piece: &Piece, old_pos: Position, new_pos: Position) -> (bool, Position) {
    // New position cannot be occupied
    let mut valid = false;
    let mut kill_pos = Position { row: -1, col: -1 };

    // TODO: Add capturing pieces
    match (piece.owner, piece.kind) {
        (Player::Red, PieceKind::Regular) => {
            if new_pos.row == old_pos.row + 1
                && (new_pos.col == old_pos.col - 1 || new_pos.col == old_pos.col + 1) {
                    valid = true;
            } 

            // Diagonal jump kill
            if new_pos.row == old_pos.row + 2
                && (new_pos.col == old_pos.col - 2 || new_pos.col == old_pos.col + 2) {
                    for (piece, pos) in pieces_vec.iter() {
                        if pos.row == new_pos.row - 1
                            && (pos.col == new_pos.col - 1 || pos.col == new_pos.col + 1)
                            && piece.owner == Player::Black {
                            valid = true;
                            kill_pos = *pos;
                            break;
                        }
                    }
            } 
        },
        // Moving any piece to 4, 1 causes a poison error
        (Player::Black, PieceKind::Regular) => {
            if new_pos.row == old_pos.row - 1
                && (new_pos.col == old_pos.col - 1 || new_pos.col == old_pos.col + 1) {
                    valid = true;
            } 

            if new_pos.row == old_pos.row - 2
                && (new_pos.col == old_pos.col - 2 || new_pos.col == old_pos.col + 2) {
                    for (piece, pos) in pieces_vec.iter() {
                        if pos.row == new_pos.row + 1
                            && (pos.col == new_pos.col - 1 || pos.col == new_pos.col + 1)
                            && piece.owner == Player::Red {
                            valid = true;
                            kill_pos = *pos;
                            break;
                        }
                    }
            } 
        },
        _ => eprintln!("Unimplemented logic at position_is_valid")
    }

    (valid, kill_pos)
}

