use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod cell;
mod scene;
use cell::*;
use scene::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, cell::spawn_cell)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, update_all_cells)
        .run();
}
