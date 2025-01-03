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
        app.add_systems(Update, (update_position, update_rotation).chain());
    }
}

// Mutates `transform` component of ALL entities by their velocity.
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.linvel * time.delta_secs();
    }
}

fn update_rotation(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        // FIXME: Allow for rotation via input from angvel
        transform.rotate_y(velocity.angvel.y * time.delta_secs())
    }
}
