use crate::{
    movement::Velocity,
    player::{Player, PlayerAction},
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const STARTING_CAMERA_DISTANCE: f32 = 30.0;
#[derive(Component)]
struct Camera;

#[derive(Bundle)]
struct CameraBundle {
    marker: Camera,
    camera: Camera3d,
    transform: Transform,
    velocity: Velocity,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(
            Update,
            (follow_player, look_around_mouse, zoom_control).chain(),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        // Camera3dBundle was depreciated.
        CameraBundle {
            marker: Camera,
            camera: Camera3d::default(),
            transform: Transform::from_xyz(0.0, STARTING_CAMERA_DISTANCE, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Z),
            velocity: Velocity {
                linvel: Vec3::ZERO,
                angvel: Quat::IDENTITY,
            },
        },
    );
}

// Copied from player.rs
fn look_around_mouse(
    // TODO: Not sure why action needs to be called with crate::
    mut query_camera: Query<&mut Transform, With<Camera>>,
    query_player: Query<&ActionState<PlayerAction>, (With<Player>, Without<Camera>)>, // w/o cam: https://bevyengine.org/learn/errors/b0001/
) {
    let mut camera_transform = query_camera.single_mut();
    let action_state = query_player.single();
    let camera_pan_vector = action_state.axis_pair(&PlayerAction::Look);
    const CAMERA_PAN_RATE: f32 = 0.05;
    info!("CAMERA LOOK VECTOR {:?}", camera_pan_vector);
    if (action_state.pressed(&PlayerAction::AllowLook)) {
        camera_transform.translation.x += CAMERA_PAN_RATE * camera_pan_vector.x;
        camera_transform.translation.z += CAMERA_PAN_RATE * camera_pan_vector.y;
    }
}

fn zoom_control(
    mut query_camera: Query<&mut Velocity, With<Camera>>,
    query_player: Query<&ActionState<PlayerAction>, (With<Player>, Without<Camera>)>, // w/o cam: https://bevyengine.org/learn/errors/b0001/
) {
    const CAMERA_ZOOM_RATE: f32 = 3.0; // has to be small number
    let mut camera_height = query_camera.single_mut();
    let action_state = query_player.single();

    let zoom_delta = action_state.value(&PlayerAction::Zoom);
    // Negative = Zoom In
    // Positive = Zoom Out
    info!("ZOOM DELTA {:?}", zoom_delta);
    camera_height.linvel.y = zoom_delta * CAMERA_ZOOM_RATE;
}

fn follow_player(
    mut query_camera: Query<&mut Velocity, With<Camera>>,
    query_player: Query<&Velocity, (With<Player>, Without<Camera>)>, // w/o cam: https://bevyengine.org/learn/errors/b0001/
) {
    // https://www.reddit.com/r/bevy/comments/1afnv3s/comment/kobak7r/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    for mut camera_velocity in &mut query_camera {
        for player_velocity in &query_player {
            camera_velocity.linvel = player_velocity.linvel;
        }
    }
}
