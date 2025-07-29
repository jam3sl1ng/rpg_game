use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::WindowPlugin;

const PLAYER_SPEED: f32 = 100.0;

const CAMERA_DECAY_RATE: f32 = 1.0; // How quickly should the camera follow the player

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
        .add_systems(Update, (move_player, update_camera).chain())
        .run();
}

////////////////////////
//////// CAMERA ////////
////////////////////////

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

pub fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Smooth effect to move the camera position towards the player
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
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

// Update the player's position based on keyboard input
pub fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if kb_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if kb_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if kb_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    // Continuously update the players position over time
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.0);
}
