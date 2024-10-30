#![allow(unused)]

mod first_person {
    pub mod spawn_env;
    pub mod spawn_player;
    pub mod gravity;
    pub mod rigidbody;
}

use bevy::prelude::*;
use first_person::spawn_player::SpawnPlayerPlugin;
use first_person::spawn_env::SpawnEnvPlugin;
use first_person::gravity::{GravityPlugin, HasGravity};
use first_person::rigidbody::{RigidBody, RigidBodyPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins((SpawnPlayerPlugin, SpawnEnvPlugin, RigidBodyPlugin));
    app.run();
}