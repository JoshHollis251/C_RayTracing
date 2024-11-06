use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::load_assets::{LoadedAssets, AssetLoadState};
use super::spawn_player::Player;

use bevy_sprite3d::*;

#[derive(Component)]
struct Spongebob;

#[derive(Component)]
struct RotateToPlayer;

pub struct BobPlugin;

impl Plugin for BobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Ready), spawn_bob);
        app.add_systems(
            Update,
             (
                handle_rotate_to_player,
                handle_bob_chase
            ).run_if(in_state(AssetLoadState::Ready))
        );
    }
}

fn spawn_bob (
    mut commands: Commands,
    mut loaded_assets: Res<LoadedAssets>,
    mut params: Sprite3dParams,
) {

    let sponge_handle = loaded_assets.assets.get("spongebob").unwrap(); 
    commands.spawn((
        Sprite3d {
            image: sponge_handle.clone(),
            transform: Transform::from_translation(Vec3::new(-17.5, -2.5, 0.0)),
            pixels_per_metre: 100.,
            double_sided: true,
            ..Default::default()
        }.bundle(&mut params),
        Spongebob,
        RotateToPlayer,
    ));
}

fn handle_rotate_to_player(
    mut query: Query<&mut Transform, (With<RotateToPlayer>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<RotateToPlayer>)>,
) {
    for mut transform in query.iter_mut() {
        for player_transform in player_query.iter() {
            transform.rotation = transform.looking_at(player_transform.translation, Vec3::Y).rotation;
        }
    }
}

fn handle_bob_chase (
    mut query: Query<&mut Transform, (With<Spongebob>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Spongebob>)>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let player_transform = player_query.single();
        let mut distance = player_transform.translation - transform.translation;
        let move_vector = distance.xz().normalize();
        transform.translation += Vec3::new(move_vector.x, 0., move_vector.y) * time.delta_seconds() * 6.5;
    }
}