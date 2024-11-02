use bevy::{prelude::*, render::view::RenderLayers};
use bevy_rapier3d::prelude::*;

pub struct BuildDungeonPlugin;

impl Plugin for BuildDungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_dungeon);
    }
}

fn build_dungeon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let floor_mesh = meshes.add(Cuboid::new(5.0, 1.0, 5.0));
    let side_wall = meshes.add(Cuboid::new(1.0, 5.0, 5.0));
    let back_wall = meshes.add(Cuboid::new(5.0, 5.0, 1.0));
    let grass_texture: Handle<Image> = asset_server.load("texture/Grass/Grass_07-512x512.png");
    let wall_texture: Handle<Image> = asset_server.load("texture/Bricks/Bricks_21-512x512.png");
    
    for x in -5..5 {
        for z in -5..5 {
            commands.spawn((
                PbrBundle {
                    mesh: floor_mesh.clone(),
                    transform: Transform::from_translation(Vec3::new((x as f32) * 5., -5.0, (z as f32) * 5.)),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(grass_texture.clone()),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                RigidBody::Fixed,
                Collider::cuboid(2.5, 0.5, 2.5)
            ));
        }
    }

    for x in -5..5 {
        commands.spawn((
            PbrBundle {
                mesh: back_wall.clone(),
                transform: Transform::from_translation(Vec3::new((x as f32) * 5., -2.5, -27.5)),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(wall_texture.clone()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            RigidBody::Fixed,
            Collider::cuboid(2.5, 2.5, 0.5),
            Friction {
                coefficient: 0.,
                ..Default::default()
            }
        ));
    }
}

