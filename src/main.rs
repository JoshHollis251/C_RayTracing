#![allow(unused)]

pub mod first_person {
    pub mod spawn_env;
    pub mod spawn_player;
    pub mod spawn_rigidbody;
    pub mod dungeon;
    pub mod spongebob;
    pub mod load_assets;
}

pub mod blender {
    pub mod spawn_stuff;
}

use bevy::prelude::*;
use std::collections::HashMap;
use bevy_rapier3d::prelude::*;
use first_person::spawn_player::SpawnPlayerPlugin;
use first_person::spawn_env::SpawnEnvPlugin;
use first_person::spawn_rigidbody::SpawnRigidbodyPlugin;
use first_person::dungeon::BuildDungeonPlugin;

use blender::spawn_stuff::SpawnStuffPlugin;

use first_person::load_assets::AssetPlugin;

use first_person::spongebob::BobPlugin;

fn main() {
    let mut asset_dict: HashMap<String, String> = HashMap::default();
    asset_dict.insert("spongebob".to_string(), "img/spongebob.png".to_string());
    asset_dict.insert("grass".to_string(), "texture/Grass/Grass_07-512x512.png".to_string());
    asset_dict.insert("bricks".to_string(), "texture/Bricks/Bricks_21-512x512.png".to_string());

    let mut app = App::new();
    app.add_plugins((DefaultPlugins, AssetPlugin::new(asset_dict), RapierPhysicsPlugin::<NoUserData>::default(), RapierDebugRenderPlugin::default()))
        .add_plugins((SpawnPlayerPlugin, BuildDungeonPlugin, BobPlugin));
        // .add_plugins(RapierDebugRenderPlugin::default());
    app.run();
}