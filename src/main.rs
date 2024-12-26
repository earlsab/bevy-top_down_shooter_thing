mod camera;
mod debug;
mod enemies;
mod movement;
mod player;
mod world;

use bevy::log::LogPlugin;
use bevy::{
    prelude::*,
    remote::{http::RemoteHttpPlugin, RemotePlugin},
};

use camera::CameraPlugin;
use debug::DebugPlugin;
use enemies::EnemyPlugin;
use leafwing_input_manager::prelude::*;
use movement::MovementPlugin;
use player::{PlayerAction, PlayerPlugin};
use world::WorldPlugin;

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
        // Plugins required for VSCode Bevy Inspector
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        // User configured plugins.
        // .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .run();
}
