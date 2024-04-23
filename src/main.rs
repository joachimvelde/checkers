use bevy::{prelude::*, window::WindowResolution};

static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 800.0;
static TILES: i32 = 8;

#[derive(Component)]
struct Tile;

#[derive(Component)]
struct Checker {
    is_king: bool,
    colour: CheckerColour
}

enum CheckerColour {
    Red,
    Black
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Checkers".to_string(),
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let tile_width: f32 = WIDTH / TILES as f32;
    let tile_height: f32 = HEIGHT / TILES as f32;

    for y in -TILES/2..TILES/2 {
        for x in -TILES/2..TILES/2 {
            let x_pos: f32 = x as f32 * tile_width + tile_width / 2.0;
            let y_pos: f32 = y as f32 * tile_height + tile_height / 2.0;

            let color;
            if (x + y) % 2 == 0 {
                color = Color::RED;
            } else {
                color = Color::GREEN;
            }

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(tile_width, tile_height)),
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                ..default()
            });
        }
    }
}
