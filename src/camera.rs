use crate::{movement::Velocity, player::Player};
use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 30.0;

#[derive(Component)]
struct Camera;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, follow_player);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(
        // Camera3dBundle was depreciated.
        (
            Camera,
            Camera3d::default(),
            Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            Velocity {
                linvel: Vec3::ZERO,
                angvel: Vec3::ZERO,
            },
        ),
    );
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
