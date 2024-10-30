#![allow(unused)] //fuck this unused shi

mod first_person {
    pub mod spawn_env;
    pub mod spawn_player;
}

use bevy::prelude::*;
use first_person::spawn_player::SpawnPlayerPlugin;
use first_person::spawn_env::SpawnEnvPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins((SpawnPlayerPlugin, SpawnEnvPlugin));
    app.run();

}