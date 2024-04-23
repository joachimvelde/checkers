use bevy::prelude::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    create_board(&mut commands);
    create_pieces(&mut commands);
}

fn create_board(commands: &mut Commands) {
    let tile_width = SCREEN_WIDTH / 8.0;
    let tile_height = SCREEN_WIDTH / 8.0;

    for row in 0..8 {
        for col in 0..8 {
            let x = tile_width / 2.0 - SCREEN_WIDTH / 2.0 + tile_width * col as f32;
            let y = tile_height / 2.0 - SCREEN_HEIGHT / 2.0 + tile_height * row as f32;

            let color;
            if (row + col) as f32 % 2.0 == 0.0 {
                color = Color::WHITE;
            } else {
                color = Color::BLACK;
            }

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(tile_width, tile_height)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            });
        }
    }
}

// TODO: Make tile_width and tile_height constants and make the x and y calculations macros
fn create_pieces(commands: &mut Commands) {
    let tile_width = SCREEN_WIDTH / 8.0;
    let tile_height = SCREEN_WIDTH / 8.0;

    for row in 0..8 {
        for col in 0..8 {
            let x = tile_width / 2.0 - SCREEN_WIDTH / 2.0 + tile_width * col as f32;
            let y = tile_height / 2.0 - SCREEN_HEIGHT / 2.0 + tile_height * row as f32;

            if row > 4 && (row + col) as f32 % 2.0 != 0.0 {
                spawn_piece(commands, Vec2::new(x, y), Color::RED);
            }

            if row < 3 && (row + col) as f32 % 2.0 != 0.0 {
                spawn_piece(commands, Vec2::new(x, y), Color::PURPLE);
            }
        }
    }
}

fn spawn_piece(commands: &mut Commands, position: Vec2, color: Color) {
    let piece_width = SCREEN_WIDTH / 16.0;
    let piece_height = SCREEN_WIDTH / 16.0;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(piece_width, piece_height)),
            ..default()
        },
        transform: Transform::from_xyz(position.x, position.y, 0.0),
        ..default()
    });
}

