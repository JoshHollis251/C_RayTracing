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
fn control_player_view (
    mut query: Query<&mut Transform, With<Player>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse: EventReader<MouseMotion>,
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut focus: ResMut<WindowFocusState>,
    time: Res<Time>, 
) {
    // Rotate the player
    let mut transform = query.single_mut(); //idrk what this does
    for motion in mouse.read() {
        let yaw = -motion.delta.x * X_SENSITIVITY;
        let pitch = -motion.delta.y * Y_SENSITIVITY;
        transform.rotate_y(yaw);
        transform.rotate_local_x(pitch);
    }

    // Move the player
    let mut direction = Vec3::ZERO;
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
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    transform.translation += direction * 5.0 * time.delta_seconds();

    



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