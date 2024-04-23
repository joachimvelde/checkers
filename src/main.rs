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

fn create_pieces(commands: &mut Commands) {
    for row in 0..8 {
        for col in 0..8 {
            if row < 3 && (row + col) as f32 % 2.0 == 0.0 {
                // TODO: Calculate positions
                spawn_piece(commands, Vec2::new(0.0, 0.0), Color::RED);
            }
        }
    }
}

fn spawn_piece(commands: &mut Commands, position: Vec2, color: Color) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
        ..default()
    });
}

