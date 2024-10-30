use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[derive(Component)]
struct Player;

#[derive(Resource, Default)]
struct WindowFocusState {
    focused: bool,
}

pub struct SpawnPlayerPlugin;

impl Plugin for SpawnPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowFocusState::default());
        app.add_systems(Startup, init_player);
        app.add_systems(Update, control_player_view);
    }
}

fn init_player(mut commands: Commands, mut focus : ResMut<WindowFocusState>) {
    commands.spawn((Player, Camera3dBundle::default()));
    focus.focused = true;
}

static X_SENSITIVITY: f32 = 0.003;
static Y_SENSITIVITY: f32 = 0.002;
static MOVE_SPEED: f32 = 5.0;
static MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.1;

fn control_player_view (
    mut query: Query<&mut Transform, With<Player>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse: EventReader<MouseMotion>,
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut focus: ResMut<WindowFocusState>,
    time: Res<Time>, 
) {
    // Rotate the player
    let mut transform = query.single_mut(); 
    let mut pitch = transform.rotation.to_euler(EulerRot::YXZ).1;

    for motion in mouse.read() {
        let yaw = -motion.delta.x * X_SENSITIVITY;
        let delta_pitch = -motion.delta.y * Y_SENSITIVITY;

        pitch = (pitch + delta_pitch).clamp(-MAX_PITCH, MAX_PITCH);

        transform.rotate_y(yaw);
        transform.rotation = Quat::from_rotation_y(transform.rotation.to_euler(EulerRot::YXZ).0) * Quat::from_rotation_x(pitch);
    }

    // Move the player
    let mut direction = Vec3::ZERO;
    let mut speed = MOVE_SPEED;
    if input.pressed(KeyCode::KeyW) {
        direction -= Vec3::Z;
    }
    if input.pressed(KeyCode::KeyS) {
        direction += Vec3::Z;
    }
    if input.pressed(KeyCode::KeyA) {
        direction -= Vec3::X;
    }
    if input.pressed(KeyCode::KeyD) {
        direction += Vec3::X;
    }
    if input.pressed(KeyCode::ShiftLeft) {
        speed *= 2.0;
    }
    if direction.length() > 0.0 {
        direction = direction.normalize();
        
        // Extract the yaw rotation (rotation around the Y axis)
        let yaw_rotation = Quat::from_rotation_y(transform.rotation.to_euler(EulerRot::YXZ).0);
        
        // Transform the direction by the yaw rotation
        let movement = yaw_rotation * direction;
        
        // Normalize the movement direction on the X-Z plane
        let flat_movement = Vec3::new(movement.x, 0.0, movement.z).normalize();
        
        // Calculate the final movement vector
        let final_movement = flat_movement * speed * time.delta_seconds();
        
        // Apply the movement to the player's transform
        transform.translation += final_movement;
    }

    // Toggle focus on Escape
    if input.just_pressed(KeyCode::Escape) {
        focus.focused = !focus.focused;
    }

    if focus.focused {
        let mut main_window = window.single_mut();
        main_window.cursor.grab_mode = CursorGrabMode::Locked;
        main_window.cursor.visible = false;
    } else {
        let mut main_window = window.single_mut();
        main_window.cursor.grab_mode = CursorGrabMode::None;
        main_window.cursor.visible = true;
    }
}