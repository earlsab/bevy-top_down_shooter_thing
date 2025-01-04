use bevy::prelude::*;
use std::f32::consts::TAU;
// TODO: Understand derive Debug
#[derive(Component, Debug)]
pub struct Velocity {
    pub linvel: Vec3,
    pub angvel: Quat,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position).chain());
    }
}

// Mutates `transform` component of ALL entities by their velocity.
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.linvel * time.delta_secs();
    }
}

// fn update_rotation(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
//     for (velocity, mut transform) in query.iter_mut() {
//         // FIXME: Allow for rotation via input from angvel
//         // transform.rotation.y = velocity.angvel.y * time.delta_secs()
//         // transform.rotate_y( * time.delta_secs())
//         // transform.rotate_local_y(velocity.angvel.y * time.delta_secs());
//         // transform.rotate_y(velocity.angvel.y);
//         let rotate_by: Quat = Quat::from_xyzw(0.0, velocity.angvel.y, 0.0, 1.0);
//         // transform.rotate(rotate_by);
//     }
// }
