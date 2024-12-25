use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::prelude::*;
use ops::atan2;

use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -5.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);
// const PLAYER_ROTATE_RATE: f32 = 1.0;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    spatial: (Transform, Visibility),
    velocity: Velocity,
    model: SceneRoot,
    marker: Player, // https://bevy-cheatbook.github.io/programming/bundle.html#creating-bundles
    input_manager: InputManagerBundle<PlayerAction>,
}
// ^^ Marker Components to filter query https://bevy-cheatbook.github.io/programming/ec.html#marker-components

// Input Control
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    #[actionlike(DualAxis)]
    Aim,
    Move,
    Jump,
    Shoot,
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
            PlayerAction::Left => Some(Dir2::X),
            PlayerAction::Right => Some(Dir2::NEG_X),
            _ => None,
        }
    }
}

// https://github.com/Leafwing-Studios/leafwing-input-manager/blob/main/examples/single_player.rs#L69
impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        let mut input_map =
            InputMap::default().with_dual_axis(PlayerAction::Aim, MouseMove::default());

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
            (player_follows_mouse_cursor, player_movement).chain(),
        );
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
        input_manager: InputManagerBundle::with_map(PlayerBundle::default_input_map()),
    });
}

fn player_follows_mouse_cursor(
    // TODO: Not sure why action needs to be called vv this way.
    mut query: Query<(&mut Transform, &ActionState<crate::PlayerAction>), With<Player>>,
) {
    let (mut player_transform, action_state) = query.single_mut();
    let player_look_vector = action_state.axis_pair(&PlayerAction::Aim);

    // https://stackoverflow.com/a/65371068
    let pos = player_transform.translation.truncate();
    let target = player_look_vector;
    let angle = (target - pos).angle_to(pos);
    player_transform.rotation = Quat::from_rotation_y(angle);
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
    // Then reconvert at the end, normalizing the magnitude
    // let net_direction = Dir2::new(direction_vector);
    player_velocity.value = Vec3::new(direction_vector.x * 5.0, 0.0, direction_vector.y * 5.0);
}
fn player_shoots() {}
