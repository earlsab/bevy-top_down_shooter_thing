use std::f32::consts::PI;

use crate::movement::Velocity;
use bevy::ecs::world;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::view::window;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::prelude::*;

const PLAYER_ROTATE_RATE: f32 = 0.5;

#[derive(Component, Debug)]
pub struct CursorPosition {
    pub position: Vec2,
}
#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    spatial: (Transform, Visibility),
    velocity: Velocity,
    model: SceneRoot,
    marker: Player, // https://bevy-cheatbook.github.io/programming/bundle.html#creating-bundles
    input_manager: InputManagerBundle<PlayerAction>,
    world_cursor_position: CursorPosition,
}
// ^^ Marker Components to filter query https://bevy-cheatbook.github.io/programming/ec.html#marker-components

// Input Control
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,

    Move,
    Jump,
    Shoot,
    Aim,
    AllowLook,
    #[actionlike(DualAxis)]
    Look,
    #[actionlike(Axis)]
    Zoom,
}

impl PlayerAction {
    // https://github.com/Leafwing-Studios/leafwing-input-manager/blob/78606fb78787c3f1c484c04c2c231e0dc778b8db/examples/single_player.rs#L39C3-L54C6
    const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    fn direction(self) -> Option<Dir2> {
        match self {
            PlayerAction::Up => Some(Dir2::Y),
            PlayerAction::Down => Some(Dir2::NEG_Y),
            PlayerAction::Left => Some(Dir2::X), // swapped to fix control
            PlayerAction::Right => Some(Dir2::NEG_X),
            _ => None,
        }
    }
}

// https://github.com/Leafwing-Studios/leafwing-input-manager/blob/main/examples/single_player.rs#L69
impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        // Camera
        input_map.insert(AllowLook, MouseButton::Right);
        input_map.insert_dual_axis(PlayerAction::Look, MouseMove::default());
        input_map.insert_axis(Zoom, MouseScrollAxis::Y.sensitivity(2.0));

        // Movement
        input_map.insert(Up, KeyCode::ArrowUp);
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Up, GamepadButton::DPadUp);

        input_map.insert(Down, KeyCode::ArrowDown);
        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Down, GamepadButton::DPadDown);

        input_map.insert(Left, KeyCode::ArrowLeft);
        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Left, GamepadButton::DPadLeft);

        input_map.insert(Right, KeyCode::ArrowRight);
        input_map.insert(Right, KeyCode::KeyD);
        input_map.insert(Right, GamepadButton::DPadRight);

        // Abilities
        input_map.insert(Shoot, MouseButton::Left);

        // Aim
        input_map
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                player_rotates_to_mouse_cursor,
                player_movement,
                cursor_position,
            )
                .chain(),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(
        (
            PlayerBundle {
                // Depreciated Spatial Bundle. See https://thisweekinbevy.com/issue/2024-10-21-async-assets-mesh-picking-and-the-bevy-linter
                spatial: (
                    Transform::IDENTITY, // setting allows model to show up at startup. setting to 0 makes model disappear until movement
                    Visibility::Visible,
                ),
                velocity: {
                    Velocity {
                        linvel: Vec3::ZERO,
                        angvel: Quat::IDENTITY,
                    }
                },
                // SceneBundle -> SceneRoot https://bevyengine.org/learn/migration-guides/0-14-to-0-15/#migrate-scenes-to-required-components
                model: SceneRoot(asset_server.load("Christmas Tree.glb#Scene0")),
                marker: Player,
                input_manager: InputManagerBundle::with_map(PlayerBundle::default_input_map()),
                world_cursor_position: {
                    CursorPosition {
                        position: Vec2::ZERO,
                    }
                },
            }
            // TODO: Add directional marker for forward facing direction for debugging
            // .insert(meshes.add(Cuboid::default()))),
        ),
    );
}

fn player_movement(
    mut query: Query<(&mut Velocity, &ActionState<crate::PlayerAction>), With<Player>>,
) {
    let (mut player_velocity, action_state) = query.single_mut();
    let mut direction_vector = Vec2::ZERO;
    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(&input_direction) {
            if let Some(direction) = input_direction.direction() {
                // Sum the directions as 2D vectors
                direction_vector += *direction;
            }
        }
    }
    // TODO: Normalize vectors for diagonals
    // Then reconvert at the end, normalizing the magnitude
    // let net_direction = Dir2::new(direction_vector);
    player_velocity.linvel = Vec3::new(direction_vector.x * 5.0, 0.0, direction_vector.y * 5.0);
}

fn player_rotates_to_mouse_cursor(
    // TODO: Not sure why action needs to be called with crate::
    mut query: Query<(&mut Transform, &mut Velocity, &CursorPosition), With<Player>>,
) {
    let (mut player_transform, mut player_velocity, world_cursor_position) = query.single_mut();
    // Still can't understand why this isn't working.
    // TODO: Allow Player to Rotate to Mouse Cursor
    player_transform.rotate_y(0.005);
}

// fn player_shoots() {}

// FIXME: Fix this mess.
fn cursor_position(
    mut query_player: Query<&mut CursorPosition, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Transform, &Camera3d), With<Camera>>,
) {
    let mut world_cursor_position = query_player.single_mut();
    let window = q_windows.single();
    let (camera_transform, camera) = q_camera.single();
    // Games typically only have one window (the primary window)
    if let Some(position) = window.cursor_position() {
        world_cursor_position.position = window_to_world(&window, &camera_transform, &position);
    } else {
        println!("Cursor is not in the game window.");
    }
}

// https://stackoverflow.com/a/65437868
// TODO: Understand how this works. Assess if necessary.
fn window_to_world(window: &Window, camera: &Transform, position: &Vec2) -> Vec2 {
    let center = camera.translation.truncate();
    let half_width = (window.width() / 2.0) * camera.scale.x;
    let half_height = (window.height() / 2.0) * camera.scale.y;
    let left = center.x - half_width;
    let bottom = center.y - half_height;
    Vec2::new(
        left + position.x * camera.scale.x,
        bottom + position.y * camera.scale.y,
    )
}
