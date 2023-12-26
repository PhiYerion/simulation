use super::cell_base::Cell;
use super::cell_bundle::{move_cell, update_cell_mesh, CellBundle};
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::geometry::Collider;

pub fn update_all_cells(
    mut cell_zip: Query<(
        &mut Cell,
        &mut Collider,
        &mut Velocity,
        &mut Mesh2dHandle,
        &mut Handle<ColorMaterial>,
    )>,
    time: Res<Time>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut color_assets: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut cell, mut collider, mut velocity, mut mesh, mut color) in cell_zip.iter_mut() {
        let cell_speed = cell.speed;
        let rand_speed = || (rand::random::<f32>() - 0.5) * cell_speed;
        let rand_vel = Vec2::new(rand_speed(), rand_speed());
        move_cell(&mut velocity, &mut cell, window.single(), rand_vel);
        update_cell_mesh(
            &mut cell,
            &mut collider,
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
    (0..100).enumerate().for_each(|_| {
        commands.spawn((CellBundle::new(
            &mut meshes,
            &mut materials,
            Vec2::new(window.width(), window.height()),
        ),));
    });
}
