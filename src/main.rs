use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};


fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))] //idk why this is needed
        Wireframe2dPlugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, move_object);
    app.run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    let shapes = [
        Mesh2dHandle(meshes.add(Capsule2d::new(10.0, 15.0))),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.iter().enumerate() {
        let color = Color::linear_rgb(1.0, 1.00, 1.0);
        
        commands.spawn(MaterialMesh2dBundle {
            mesh: shape.clone(),
            material: materials.add(color),
            transform: Transform::from_translation(Vec3::new((i as f32 - num_shapes as f32 / 2.0) * 100.0, 0.0, 0.0)),
            ..default()
        });


    }
}

static MOVE_SPEED: f32 = 7.0;

fn move_object(mut keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut Transform, &Mesh2dHandle)>) { //move
    let mut move_speed = MOVE_SPEED;
    
    //shift make go faster
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        move_speed *= 2.0;
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        move_speed *= 0.5;

    } else {

    }
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::KeyW) {
        //move objects up
        for (mut transform, shape) in query.iter_mut() {
            transform.translation.y += move_speed;
        }
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::KeyS) {
        //move objects down
        for (mut transform, shape) in query.iter_mut() {
            transform.translation.y -= move_speed;
            
        }
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::KeyA) {
        //move objects left
        for (mut transform, shape) in query.iter_mut() {
            transform.translation.x -= move_speed;
        }
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::KeyD) {
        //move objects right
        for (mut transform, shape) in query.iter_mut() {
            transform.translation.x += move_speed;
        }
    }
}

// KeyboardInput example output
// KeyboardInput { key_code: KeyA, logical_key: Character("a"), state: Pressed, window: Entity { index: 0, generation: 1 } }
// KeyboardInput { key_code: KeyS, logical_key: Character("s"), state: Released, window: Entity { index: 0, generation: 1 } }