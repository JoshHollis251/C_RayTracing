#![allow(unused)]

mod first_person {
    pub mod spawn_env;
    pub mod spawn_player;
    pub mod spawn_rigidbody;
    pub mod dungeon;
}

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use first_person::spawn_player::SpawnPlayerPlugin;
use first_person::spawn_env::SpawnEnvPlugin;
use first_person::spawn_rigidbody::SpawnRigidbodyPlugin;
use first_person::dungeon::BuildDungeonPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default()))
        .add_plugins((SpawnPlayerPlugin, BuildDungeonPlugin));
    app.run();
}