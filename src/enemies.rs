use bevy::prelude::*;

use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(5.0, 0.0, 5.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    spatial: (Transform, Visibility),
    velocity: Velocity,
    model: SceneRoot,
    marker: Enemy,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
    }
}

// fn spawn_jitter(vec3: Vec3) {

// }

// Copied from player.rs
fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(EnemyBundle {
        // Depreciated Spatial Bundle. See https://thisweekinbevy.com/issue/2024-10-21-async-assets-mesh-picking-and-the-bevy-linter
        spatial: (
            Transform::from_translation(STARTING_TRANSLATION),
            Visibility::default(),
        ),
        velocity: {
            Velocity {
                linvel: STARTING_VELOCITY,
                angvel: Vec3::ZERO,
            }
        },
        // SceneBundle -> SceneRoot https://bevyengine.org/learn/migration-guides/0-14-to-0-15/#migrate-scenes-to-required-components
        model: SceneRoot(asset_server.load("Snowman.glb#Scene0")),
        marker: Enemy,
    });
}
