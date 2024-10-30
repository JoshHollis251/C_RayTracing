use bevy::prelude::*;

#[derive(Component, PartialEq)] // partialeq allows us to compare the enum
enum ObjectType {
    Player,
    Static
}
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)//runs setup on Startup
        .add_systems(Update, control); //runs control on Update
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) { //sprites rendered bottum-up
    commands.spawn(Camera2dBundle::default()); //camera is spawned
    commands.spawn(
        (SpriteBundle { //spawns pineapple
            texture: asset_server.load("img/pineapple.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(200., 300.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(200., 100., 0.)),
            ..default()
        },
        ObjectType::Static,
    ));
    commands.spawn( //spawns player
        (SpriteBundle {
            texture: asset_server.load("img/spongebob.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            ..default()
        },
        ObjectType::Player,
    ));
    commands.spawn((TextBundle::from_section(
        "WASD to move, Space to declare yourself",
        TextStyle {
            font: asset_server.load("font/sponge.otf"),
            font_size: 40.0,
            color: Color::WHITE,
            ..default()
        }
    )).with_text_justify(JustifyText::Center));
}

static MOVE_SPEED: f32 = 5.;

fn control(mut query: Query<(&mut ObjectType, &mut Transform)>, //fetches objects with transform and object type
    input: Res<ButtonInput<KeyCode>>, 
    asset_server: Res<AssetServer>, //loads audio
    mut commands: Commands) { //enables spawns
    for (object_type, mut transform) in query.iter_mut() {
        if *object_type == ObjectType::Player {
            if input.pressed(KeyCode::KeyW) {
                transform.translation.y += MOVE_SPEED;
            }
            if input.pressed(KeyCode::KeyS) {
                transform.translation.y -= MOVE_SPEED;
            }
            if input.pressed(KeyCode::KeyA) {
                transform.translation.x -= MOVE_SPEED;
            }
            if input.pressed(KeyCode::KeyD) {
                transform.translation.x += MOVE_SPEED;
            }
        }
    }
    if input.just_pressed(KeyCode::Space) {
        commands.spawn( AudioBundle {
            source: asset_server.load("sound/bob.ogg"), //only use ogg apparently
            ..default()
        });
    }
}