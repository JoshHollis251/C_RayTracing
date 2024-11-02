use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::ColliderShape};
pub struct SpawnStuffPlugin;

impl Plugin for SpawnStuffPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_thing);
    }
}

fn spawn_thing(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let glb_handle = asset_server.load("levels/Scene.glb#Scene0");
    let prim = asset_server.load("levels/Scene.glb#Mesh0/Primitive0");

    commands.spawn(PbrBundle {
        mesh: prim,
        transform: Transform::from_translation(Vec3::new(5.0, 0.0, 5.0)),
        ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(1.0, 1.0, 1.0))
    .insert(PointLightBundle::default());
    
    commands.spawn( SceneBundle {
        scene: glb_handle,
        ..Default::default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(1.0, 1.0, 1.0))
    .insert(PointLightBundle::default());


    // commands.spawn((
    //     BlueprintInfo::from_path("levels/Scene.glb"),
    //     SpawnBlueprint,
    // ));    
}