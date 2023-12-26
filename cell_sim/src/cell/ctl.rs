use super::cell_base::Cell;
use super::cell_bundle::{move_cell, update_cell_mesh, CellBundle};
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::Mesh2dHandle};

pub fn update_all_cells(
    mut cell_zip: Query<(
        &mut Cell,
        &mut Transform,
        &mut Mesh2dHandle,
        &mut Handle<ColorMaterial>,
    )>,
    time: Res<Time>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut color_assets: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut cell, mut transform, mut mesh, mut color) in cell_zip.iter_mut() {
        let cell_speed = cell.speed;
        let rand_speed = || (rand::random::<f32>() - 0.5) * cell_speed;
        let rand_vel = Vec2::new(rand_speed(), rand_speed());
        move_cell(&mut transform, &mut cell, window.single(), rand_vel);
        update_cell_mesh(
            &mut cell,
            &mut mesh,
            &mut color,
            &mut mesh_assets,
            &mut color_assets,
        );
        cell.update(time.delta_seconds());
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
