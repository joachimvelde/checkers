use bevy::{
    prelude::*,
    input::mouse::MouseButton,
    window::{PrimaryWindow, WindowResolution}
};

const SCREEN_SIZE: f32 = 800.0;
const TILE_SIZE: f32 = SCREEN_SIZE / 8.0;
const PIECE_SIZE: f32 = TILE_SIZE / 2.0;

#[derive(Debug)]
enum PieceKind {
    Regular,
    King
}

#[derive(Debug)]
enum Player {
    Red,
    Black
}

#[derive(Component, Debug)]
struct Piece {
    kind: PieceKind,
    owner: Player
}

#[derive(Component)]
struct Position {
    row: usize,
    col: usize
}

#[derive(Component)]
struct Tile;

#[derive(Resource)]
struct DragState {
    piece: Option<Entity>,
    initial_position: Vec2,
    offset: Vec2
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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for row in 0..8 {
        for col in 0..8 {
            let x = -SCREEN_SIZE / 2.0 + TILE_SIZE / 2.0 + TILE_SIZE * col as f32;
            let y = -SCREEN_SIZE / 2.0 + TILE_SIZE / 2.0 + TILE_SIZE * (7 - row) as f32;

            // Create tiles
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
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert(Tile)
            .insert(Position{ row, col });

            // Create pieces
            if row < 3 && (row + col) as f32 % 2.0 != 0.0 {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(PIECE_SIZE, PIECE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
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
                    transform: Transform::from_xyz(x, y, 0.0),
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
    left_mouse: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    tiles: Query<(&Tile, &Position, &Transform)>,
    mut pieces: Query<(Entity, &Piece, &Position, &Transform)>,
    mut drag: ResMut<DragState>
)
{
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
                        if mouse_position.distance(center) <= PIECE_SIZE {
                            eprintln!("{:?} clicked!", piece);
                            drag.piece = Some(entity);
                        }
                    }
                }

                // Check for drags
                if left_mouse.pressed(MouseButton::Left) {
                    if let Some(entity) = drag.piece {
                        if let Ok(transform) = pieces.get_mut(entity) {
                            eprintln!("Dragging");
                        }
                    }
                }
            }
}
