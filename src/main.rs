use bevy::prelude::*;
use dungeon_example::{map::spawn_camera, player::spawn_player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player))
    .run();
}
