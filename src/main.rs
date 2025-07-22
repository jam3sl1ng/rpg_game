use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::WindowPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(
                    WindowPlugin {
                        primary_window: Some(Window {
                            title: "RPG Game".to_string(),
                            ..default()
                        }),
                        ..default()
                    },
                )
                .set(
                    ImagePlugin::default_nearest(),
                )
        )
        .insert_resource(ClearColor(Color::srgb(155.0/255.0, 212.0/255.0, 195.0/255.0)))
        .add_systems(Startup, (setup_camera, spawn_player))
        .run();
}

fn setup_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Spawn a 2D camera in the center of the primary window
    if let Ok(window) = window_query.single() {
        commands.spawn((
            Camera2d::default(),
            Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0,
            ),
        ));
    } else {
        error!("Primary window not found!");
    }
}

////////////////////////
/// PLAYER COMPONENT ///
////////////////////////

#[derive(Component)]
pub struct Player {

}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(window) = window_query.single() {
        commands.spawn((
            Sprite {
                image: asset_server.load("characters/player/move_down_0.png"),
                ..default()
            },
            Transform {
                // Spawn the player in the center of the window
                translation: Vec3::new(
                    window.width() / 2.0,
                    window.height() / 2.0,
                    0.0,
                ),
                scale: Vec3::splat(4.0), // Scale the sprite up by 2
                ..default()
            },
            Player {},
        ));
    } else {
        error!("Primary window not found!");
    }
}