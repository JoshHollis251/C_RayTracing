use bevy::prelude::*;

pub struct SpawnEnvPlugin;

impl Plugin for SpawnEnvPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_env);
        
    }
}

fn build_env(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    let cube = meshes.add(Cuboid::new(1.0, 1.0,1.0));
    let texture: Handle<Image> = asset_server.load("img/spongebob.png");
    for x in -5..5 {
        for z in -5..5 {
            commands.spawn( PbrBundle {
                mesh: cube.clone(),
                transform: Transform::from_translation(Vec3::new((x as f32) * 3., -5.0, (z as f32) * 3.)),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(texture.clone()),
                    ..Default::default()
                }),
                ..default()
            });
            commands.spawn( PbrBundle {
                mesh: cube.clone(),
                transform: Transform::from_translation(Vec3::new((x as f32) * 3., 5.0, (z as f32) * 3.)),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                ..default()
            });
        }
    }
}