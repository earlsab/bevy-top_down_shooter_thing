use bevy::prelude::*;

use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -5.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    spatial: (Transform, Visibility),
    velocity: Velocity,
    model: SceneRoot,
    marker: Player, // https://bevy-cheatbook.github.io/programming/bundle.html#creating-bundles
}
// ^^ Marker Components to filter query https://bevy-cheatbook.github.io/programming/ec.html#marker-components

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle {
        // Depreciated Spatial Bundle. See https://thisweekinbevy.com/issue/2024-10-21-async-assets-mesh-picking-and-the-bevy-linter
        spatial: (
            Transform::from_translation(STARTING_TRANSLATION),
            Visibility::default(),
        ),
        velocity: {
            Velocity {
                value: STARTING_VELOCITY,
            }
        },
        // SceneBundle -> SceneRoot https://bevyengine.org/learn/migration-guides/0-14-to-0-15/#migrate-scenes-to-required-components
        model: SceneRoot(asset_server.load("Christmas Tree.glb#Scene0")),
        marker: Player,
    });
}
