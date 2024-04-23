use bevy::prelude::*;

const SCREEN_SIZE: f32 = 800.0;
const TILE_SIZE: f32 = SCREEN_SIZE / 8.0;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Red,
    Black,
    RedKing,
    BlackKing
}

#[derive(Resource)]
struct Board {
    pieces: [[Tile; 8]; 8]
}

impl Board {
    fn new() -> Board {
        let mut pieces = [[Tile::Empty; 8]; 8];

        for row in 0..8 {
            for col in 0..8 {
                // TODO: Add kings
                if row > 4 && (row + col) as f32 % 2.0 != 0.0 {
                    pieces[row][col] = Tile::Red;
                }
                if row < 3 && (row + col) as f32 % 2.0 != 0.0  {
                    pieces[row][col] = Tile::Black;
                }
            }
        }

        Board { pieces }
    }

    fn render(&self, commands: &mut Commands) {
        for row in 0..8 {
            for col in 0..8 {
                let x = TILE_SIZE / 2.0 - SCREEN_SIZE / 2.0 + TILE_SIZE * col as f32;
                let y = TILE_SIZE / 2.0 - SCREEN_SIZE / 2.0 + TILE_SIZE * row as f32;

                // Render the board
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
                });

                // Render the pieces
                match self.pieces[row][col] {
                    Tile::Red => {
                        commands.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::RED,
                                custom_size: Some(Vec2::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(x, y, 0.0),
                            ..default()
                        });
                    },
                    Tile::Black => {
                        commands.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::PURPLE,
                                custom_size: Some(Vec2::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(x, y, 0.0),
                            ..default()
                        });
                    },
                    _ => ()
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
