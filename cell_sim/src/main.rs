use bevy::window::PrimaryWindow;
use bevy::{log, prelude::*};
mod cell;
mod scene;
use scene::spawn_camera;
use cell::move_all_cells;

use self::cell::grow_cell;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, cell::spawn_cell)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, grow_cell)
        .run();
}

