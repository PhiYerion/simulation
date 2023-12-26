use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy::window::PrimaryWindow;
use super::cell_base::{Cell, move_cell, grow_cell, CellBundle};

pub fn update_all_cells(
    mut cell_zip: Query<(&mut Cell, &mut Transform, &mut Mesh2dHandle)>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut cell, mut transform, mut mesh) in cell_zip.iter_mut() {
        let cell_speed = cell.speed;
        let rand_speed = || (rand::random::<f32>() - 0.5) * cell_speed;
        let rand_vel = Vec2::new(rand_speed(), rand_speed());
        move_cell(&mut transform, &mut cell, window.single(), rand_vel);
        grow_cell(&mut cell, &mut mesh, &mut mesh_assets, 0.01)
    }
}

pub fn spawn_cell(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    commands.spawn((CellBundle::new(
        &mut meshes,
        &mut materials,
        Vec2::new(window.width(), window.height()),
    ),));
}
