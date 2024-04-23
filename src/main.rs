use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

static WIDTH: f32 = 1000.0;
static HEIGHT: f32 = 1000.0;
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
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(Camera2dBundle::default());

    let tile_width: f32 = WIDTH / TILES as f32;
    let tile_height: f32 = HEIGHT / TILES as f32;

    for x in 0..TILES {
        for y in 0..TILES {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                material: materials.add(Color::PURPLE),
                ..default()
            });
        }
    }
}
