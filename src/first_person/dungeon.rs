use bevy::log::tracing_subscriber::fmt::time;
use bevy::{prelude::*, render::view::RenderLayers};
use bevy::asset::LoadState;
use bevy_rapier3d::prelude::*;
use bevy_sprite3d::*;
use super::spawn_player::Player;
use super::load_assets::*;

pub struct BuildDungeonPlugin;

impl Plugin for BuildDungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Sprite3dPlugin);
        app.add_systems(OnEnter(AssetLoadState::Ready), build_dungeon);
    }
}

fn build_dungeon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<LoadedAssets>,
) {
    let floor_mesh = meshes.add(Cuboid::new(5.0, 1.0, 5.0));
    let side_wall = meshes.add(Cuboid::new(1.0, 5.0, 5.0));
    let back_wall = meshes.add(Cuboid::new(5.0, 5.0, 1.0));
    let grass_texture: Handle<Image> = assets.assets.get("grass").unwrap().clone();
    let wall_texture: Handle<Image> = assets.assets.get("bricks").unwrap().clone();
    
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