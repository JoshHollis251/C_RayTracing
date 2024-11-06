use std::thread::spawn;

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_rapier3d::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy::animation::*;

use super::load_assets::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Gun;

#[derive(Component)]
struct Flash {
    ttl: f32
}

#[derive(Component)]
struct Bullet {
    ttl: f32
}

#[derive(Resource, Default)]
struct WindowFocusState {
    focused: bool,
}

pub struct SpawnPlayerPlugin;

impl Plugin for SpawnPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowFocusState::default());
        app.add_systems(OnEnter(AssetLoadState::Ready), init_player);
        app.add_systems(
            Update, 
            (
                control_player_view,
                check_gun
            ).run_if(in_state(AssetLoadState::Ready))
        );
    }
}

#[derive(Resource)]
struct Animations {
    animations: AnimationNodeIndex,
    #[allow(dead_code)]
    graph: Handle<AnimationGraph>,
}

fn init_player(
    mut commands: Commands, 
    mut focus : ResMut<WindowFocusState>, 
    mut asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let bob: Handle<Image> = asset_server.load("img/spongebob.png");
    commands.spawn((
        Player,
        RigidBody::Dynamic,
        SpatialBundle::default()
    ))
        .with_children(|parent| {
            parent.spawn((SceneBundle {
                    scene: asset_server.load("blueprints/player.glb#Scene0"),
                    transform: Transform::from_scale(Vec3::new(0.2, 0.2, 0.2)) * 
                        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)) * 
                        Transform::from_translation(Vec3::new(0.0, -7., 0.0)),
                    visibility: Visibility::Hidden,
                    ..default()
                },
            ));
            parent.spawn( (Camera3dBundle {
                    projection: PerspectiveProjection {
                        fov: 90.0_f32.to_radians(),
                        ..default()
                    }.into(),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                    ..default()
                },
            ))
                .with_children(|camera_subparent| {
                    camera_subparent.spawn((
                        Gun,
                        SpatialBundle{
                            transform: Transform::from_translation(Vec3::new(1.0, -0.5, -1.0)),
                            ..default()
                        },
                ))
                    .with_children(|gun_subparent| {
                        gun_subparent.spawn(
                            SceneBundle {
                                scene: asset_server.load("blueprints/revolver.glb#Scene0"),
                                transform: Transform::from_translation(Vec3::new(0., 0., 0.)) * 
                                    Transform::from_rotation(Quat::from_rotation_y(3. * std::f32::consts::FRAC_PI_2)),
                                ..default()
                            },
                        );
                        gun_subparent.spawn((
                            Flash{ttl: 0.1},
                            PbrBundle {
                                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                                material: materials.add(StandardMaterial {
                                    base_color: Color::srgb(1.0, 1.0, 1.0),
                                    unlit: true,
                                    emissive: LinearRgba { red: (10.), green: (10.), blue: (10.), alpha: (1.) },
                                    emissive_exposure_weight: 10.0,

                                    ..default()
                                }),
                                transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
                                ..default()
                            },
                        ));
                    });
                });
            parent.spawn( PointLightBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 6.0, 0.0)),
                point_light: PointLight {
                    intensity: 1000000.0,
                    range: 10000.0,
                    color: Color::srgb(1., 1., 1.),
                    ..default()
                },
                ..default()
            }); // light

        })
        .insert(Collider::capsule_y(0.75, 0.75))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::default())
        .insert(GravityScale(1.0))
        .insert(Friction {
            coefficient: 0.0,
            ..default()
        });
    focus.focused = true;
}

static X_SENSITIVITY: f32 = 0.003;
static Y_SENSITIVITY: f32 = 0.002;
static MOVE_SPEED: f32 = 5.0;
static MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.1;

fn control_player_view (
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut GravityScale), (With<Player>, Without<Camera>)>,
    mut text: Query<&mut Text>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse: EventReader<MouseMotion>,
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut focus: ResMut<WindowFocusState>,
    time: Res<Time>, 
) {
    // Rotate the camera
    if camera.iter().count() == 0 {
        return;
    }
    let mut camera_transform = camera.single_mut(); 
    let (mut player_transform, mut player_velocity, mut player_gravity) = player.single_mut();
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
        let flat_movement = Vec3::new(movement.x, 0.0, movement.z).normalize() * speed;
        let final_movement = Vec3::new(flat_movement.x, player_velocity.linvel.y, flat_movement.z);
        player_velocity.linvel = final_movement;
    } else {
        player_velocity.linvel = Vec3::new(0.0, player_velocity.linvel.y, 0.0);
    }

    if input.just_pressed(KeyCode::Space) {
        player_velocity.linvel.y = 7.;
    }

    if input.pressed(KeyCode::Space) && player_velocity.linvel.y > 0.0 {
        player_gravity.0 = 1.0;
    } else {
        player_gravity.0 = 2.0;
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

fn check_gun(
    mut commands: Commands,
    mut gun : Query<&GlobalTransform, With<Gun>>,
    mut player_transform: Query<&GlobalTransform, With<Camera>>,
    mut bullets: Query<(&mut Bullet, Entity)>,
    mut flash: Query<(&mut Flash, Entity)>,
    mut input: ResMut<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
) {
    let player_forward = player_transform.single_mut().forward();
    if input.just_pressed(MouseButton::Left) {
        for gun_transform in gun.iter_mut() {
            spawn_bullet(&mut commands, &mut meshes, gun_transform.clone(), player_forward);
        }
    }
    for (mut bullet, entity) in bullets.iter_mut() {
        bullet.ttl -= time.delta_seconds();
        if bullet.ttl <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_bullet(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    transform: GlobalTransform,
    direction: Dir3
) {
    // println!("Spawning bullet at {}", transform.compute_transform().translation);
    commands.spawn((
        Bullet{ttl: 2.},
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
            transform: transform.compute_transform(),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(0.1, 0.1, 0.1),
        Velocity::linear(100. * direction),
        GravityScale(0.25),
        Friction {
            coefficient: 0.0,
            ..default()
        }
    ));
}