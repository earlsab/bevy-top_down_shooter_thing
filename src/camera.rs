use crate::{
    movement::Velocity,
    player::{Player, PlayerAction},
};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const STARTING_CAMERA_DISTANCE: f32 = 30.0;
const MAXIMUM_CAMERA_DISTANCE: f32 = 60.0;
const MINIMUM_CAMERA_DISTANCE: f32 = 10.0;

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
    // assert!(MAXIMUM_CAMERA_DISTANCE > MINIMUM_CAMERA_DISTANCE);
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
    // info!("CAMERA LOOK VECTOR {:?}", camera_pan_vector);
    if action_state.pressed(&PlayerAction::AllowLook) {
        camera_transform.translation.x += CAMERA_PAN_RATE * camera_pan_vector.x;
        camera_transform.translation.z += CAMERA_PAN_RATE * camera_pan_vector.y;
    }
}

fn zoom_control(
    mut query_camera: Query<(&Transform, &mut Velocity), With<Camera>>,
    query_player: Query<&ActionState<PlayerAction>, (With<Player>, Without<Camera>)>, // w/o cam: https://bevyengine.org/learn/errors/b0001/
) {
    const CAMERA_ZOOM_RATE: f32 = 3.0; // has to be small number
    let (camera_height, mut camera_height_velocity) = query_camera.single_mut();
    let action_state = query_player.single();

    let zoom_delta = action_state.value(&PlayerAction::Zoom);
    // Negative = Zoom In
    // Positive = Zoom Out
    // info!("ZOOM DELTA {:?}", zoom_delta);

    let mut max_excess_distance = camera_height.translation.y - MAXIMUM_CAMERA_DISTANCE;
    max_excess_distance = max_excess_distance.round();
    // e.g.
    // min 30 - 0 = 30
    // min -30 - (-)60 =
    let mut min_excess_distance = MINIMUM_CAMERA_DISTANCE - camera_height.translation.y;
    min_excess_distance = min_excess_distance.round();

    // info!("CAMERA HEIGHT {:?}", camera_height.translation.y);
    // info!("Min Ex {:?}", min_excess_distance);
    // info!("Max Ex {:?}", max_excess_distance);

    // If > 0 = exceeded max or min
    // info!("max_excess_distance {:?}", max_excess_distance);
    // info!("min_excess_distance {:?}", min_excess_distance);
    if min_excess_distance <= 0.0 && max_excess_distance <= 0.0 {
        camera_height_velocity.linvel.y = zoom_delta * CAMERA_ZOOM_RATE;
    } else {
        // Too zoomed out
        if max_excess_distance > 0.0 {
            camera_height_velocity.linvel.y = -max_excess_distance * CAMERA_ZOOM_RATE * 2.0;
        } else if min_excess_distance > 0.0 {
            camera_height_velocity.linvel.y = min_excess_distance * CAMERA_ZOOM_RATE * 2.0;
        }
    }
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
