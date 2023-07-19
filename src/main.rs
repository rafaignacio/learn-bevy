use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
struct Enemy;

pub mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugins::camera::CameraPlugin)
        .add_systems(Startup, add_enemy)
        .run();
}

fn add_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Enemy,
    ));
}
