use crate::movement::Velocity;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::prelude::*;

// const PLAYER_ROTATE_RATE: f32 = 0.5;

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
    _meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(PlayerBundle {
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
    });
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
    let (mut player_transform, _player_velocity, world_cursor_position) = query.single_mut();
    // Still can't understand why this isn't working.
    // TODO: Allow Player to Rotate to Mouse Cursor
    // player_transform.rotate_y(0.005);

    let _player_pos = player_transform.translation.truncate();
    let world_cursor_pos: Vec3 = Vec3::new(
        world_cursor_position.position.x,
        0.0,
        world_cursor_position.position.y,
    );
    // player_transform.rotation.w = player_pos.angle_to(world_cursor_pos);
    *player_transform = player_transform.looking_at(world_cursor_pos, Vec3::Y);
    // info!("{:?}", player_velocity.angvel.y);
}

// fn player_shoots() {}

fn cursor_position(
    mut query_player: Query<&mut CursorPosition, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Transform, &Camera3d), With<Camera>>,
) {
    let mut world_cursor_position = query_player.single_mut();
    let window = q_windows.single();
    let (camera_transform, _camera) = q_camera.single();
    // Games typically only have one window (the primary window)
    if let Some(position) = window.cursor_position() {
        world_cursor_position.position = window_to_world(window, camera_transform, &position);
        // info!("{:?}", world_cursor_position.position);
    } else {
        // println!("Cursor is not in the game window.");
    }
}
// FIXME: Fix this mess.
// fn cursor_position(
//     mut query_player: Query<&mut CursorPosition, With<Player>>,
//     q_windows: Query<&Window, With<PrimaryWindow>>,
//     q_camera: Query<(&Transform, &Camera3d), With<Camera>>,
// ) {
//     let mut screen_cursor_position = query_player.single_mut();
//     let window = q_windows.single();
//     let (camera_transform, camera) = q_camera.single();
//     // Games typically only have one window (the primary window)
//     if let Some(position) = window.cursor_position() {
//         screen_cursor_position.position = window_to_world(&window, &camera_transform, &position);
//     } else {
//         println!("Cursor is not in the game window.");
//     }
// }

// // https://stackoverflow.com/a/65437868
// // TODO: Understand how this works. Assess if necessary.
// AI-generated code to convert from 2d cameras to 3d. Claude 3.5 Sonnet
fn window_to_world(window: &Window, camera_transform: &Transform, cursor_pos: &Vec2) -> Vec2 {
    // Convert screen position to normalized device coordinates (NDC)
    let ndc_x = (2.0 * cursor_pos.x) / window.width() - 1.0;
    let ndc_y = 1.0 - (2.0 * cursor_pos.y) / window.height();

    // Convert to view space
    let view_x = ndc_x;
    let view_y = ndc_y;
    let view_z = 1.0; // Project onto the far plane

    // Convert to world space
    let camera_forward = camera_transform.forward();
    let camera_right = camera_transform.right();
    let camera_up = camera_transform.up();

    let world_pos = camera_transform.translation
        + camera_right * view_x
        + camera_up * view_y
        + camera_forward * view_z;

    // Calculate intersection with XZ plane (Y = 0)
    let ray_direction = (world_pos - camera_transform.translation).normalize();
    let t = -camera_transform.translation.y / ray_direction.y;
    let intersection = camera_transform.translation + ray_direction * t;

    // Return XZ coordinates as Vec2
    Vec2::new(intersection.x, intersection.z)
}
