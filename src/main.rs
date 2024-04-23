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
}

fn create_board(commands: &mut Commands) {
    let tile_width = SCREEN_WIDTH / 8.0;
    let tile_height = SCREEN_WIDTH / 8.0;

    for i in 0..8 {
        for j in 0..8 {
            let x = tile_width / 2.0 - SCREEN_WIDTH / 2.0 + tile_width * j as f32;
            let y = tile_height / 2.0 - SCREEN_HEIGHT / 2.0 + tile_height * i as f32;

            let color;
            if (i + j) as f32 % 2.0 == 0.0 {
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
