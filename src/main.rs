mod camera;
mod debug;
mod enemies;
mod movement;
mod player;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use enemies::EnemyPlugin;
use leafwing_input_manager::prelude::*;
use movement::MovementPlugin;
use player::{PlayerAction, PlayerPlugin};

fn main() {
    App::new()
        // Resources
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 800.0,
        })
        // Built in plugins
        .add_plugins(DefaultPlugins) // Default Plugins required to run game
        // User configured plugins.
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .run();
}
