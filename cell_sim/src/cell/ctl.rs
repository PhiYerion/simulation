use super::cell_base::Cell;
use super::cell_bundle::{move_cell, update_cell_mesh, update_cell_physics, CellBundle};
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::dynamics::{Damping, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties};

type CellZip<'a> = (
        &'a mut Cell,
        &'a mut Collider,
        &'a mut Velocity,
        &'a mut Mesh2dHandle,
        &'a mut Handle<ColorMaterial>,
        &'a mut ColliderMassProperties,
        &'a mut Damping,
);

pub fn update_all_cells(
    mut cell_zip: Query<CellZip>,
    time: Res<Time>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut color_assets: ResMut<Assets<ColorMaterial>>,
) {
    for (
        mut cell,
        mut collider,
        mut velocity,
        mut mesh,
        mut color,
        mut collider_mass_properties,
        mut damping,
    ) in cell_zip.iter_mut()
    {
        let cell_speed = cell.speed;
        let rand_speed = || (rand::random::<f32>() - 0.5) * cell_speed;
        let rand_vel = Vec2::new(rand_speed(), rand_speed());
        move_cell(&mut velocity, &mut cell, rand_vel);
        update_cell_mesh(
            &mut cell,
            &mut mesh,
            &mut color,
            &mut mesh_assets,
            &mut color_assets,
        );
        update_cell_physics(
            &cell,
            &mut collider,
            &mut collider_mass_properties,
            &mut damping,
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
