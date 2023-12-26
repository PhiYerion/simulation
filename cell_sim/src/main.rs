use bevy::prelude::*;
mod cell;
mod scene;
use scene::spawn_camera;
use cell::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, cell::spawn_cell)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, update_all_cells)
        .run();
}

