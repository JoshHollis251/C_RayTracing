use bevy::prelude::*;

pub struct SpawnEnvPlugin;

impl Plugin for SpawnEnvPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_env);
        
    }
}

fn build_env(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let cube = meshes.add(Cuboid::new(1.0, 1.0,1.0));
    for x in -5..5 {
        for z in -5..5 {
            commands.spawn( PbrBundle {
                mesh: cube.clone(),
                transform: Transform::from_translation(Vec3::new((x as f32) * 3., -5.0, (z as f32) * 3.)),
                ..default()
            });
        }
    }
}