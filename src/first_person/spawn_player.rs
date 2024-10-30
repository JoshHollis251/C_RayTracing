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

fn init_player(
    mut commands: Commands, 
    mut focus : ResMut<WindowFocusState>, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Player,
        SpatialBundle::default()
    ))
        .with_children(|parent| {
            parent.spawn(PbrBundle { // feet
                mesh: meshes.add(Cuboid::new(0.3, 0.3, 0.5)),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_translation(Vec3::new(0.0, -3.0, 0.0)), // Position the cube at the player's feet
                ..default()
            });
            parent.spawn( Camera3dBundle::default()); // camera
        });
    focus.focused = true;
}

static X_SENSITIVITY: f32 = 0.003;
static Y_SENSITIVITY: f32 = 0.002;
static MOVE_SPEED: f32 = 5.0;
static MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.1;

fn control_player_view (
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>, //fcking ridiculous
    mut player: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse: EventReader<MouseMotion>,
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut focus: ResMut<WindowFocusState>,
    time: Res<Time>, 
) {
    // Rotate the camera
    let mut camera_transform = camera.single_mut(); 
    let mut player_transform = player.single_mut();
    let mut pitch = camera_transform.rotation.to_euler(EulerRot::YXZ).1;

    for motion in mouse.read() {
        let yaw = -motion.delta.x * X_SENSITIVITY;
        let delta_pitch = -motion.delta.y * Y_SENSITIVITY;

        pitch = (pitch + delta_pitch).clamp(-MAX_PITCH, MAX_PITCH);

        player_transform.rotate_y(yaw);
        camera_transform.rotation = Quat::from_rotation_y(camera_transform.rotation.to_euler(EulerRot::YXZ).0) * Quat::from_rotation_x(pitch);
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
        let yaw_rotation = Quat::from_rotation_y(player_transform.rotation.to_euler(EulerRot::YXZ).0);
        let movement = yaw_rotation * direction;
        let flat_movement = Vec3::new(movement.x, 0.0, movement.z).normalize();
        let final_movement = flat_movement * speed * time.delta_seconds();
        player_transform.translation += final_movement;
    }

    // Up and Down Movement
    if input.pressed(KeyCode::Space) {
        player_transform.translation += Vec3::Y * speed * time.delta_seconds();
    }
    if input.pressed(KeyCode::ControlLeft) {
        player_transform.translation -= Vec3::Y * speed * time.delta_seconds();
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