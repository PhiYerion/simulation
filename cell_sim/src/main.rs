use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
mod cell;
mod scene;
use bevy::core::TaskPoolThreadAssignmentPolicy;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use cell::*;
use scene::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(TaskPoolPlugin {
            task_pool_options: TaskPoolOptions {
                io: TaskPoolThreadAssignmentPolicy {
                    min_threads: 1,
                    max_threads: 2,
                    percent: 0.2,
                },
                ..Default::default()
            },
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, cell::spawn_cells)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, update_all_cells)
        .run();
}
