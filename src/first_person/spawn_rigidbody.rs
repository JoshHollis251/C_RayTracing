use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct SpawnRigidbodyPlugin;

impl Plugin for SpawnRigidbodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_rigidbody);
    }
}

fn spawn_rigidbody(mut commands: Commands, mut meshes : ResMut<Assets<Mesh>>, mut materials : ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        RigidBody::Dynamic,
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        }
    ))
    .insert(Collider::capsule_y(10., 10.))
    .insert(GravityScale(1.0));

}