use super::cell_base::{Cell, CellComponent, CellComponentType, CellData};
use super::cell_bundle::{move_cell, update_cell_mesh, update_cell_physics, CellBundle};
use super::cell_components::{
    burn_glucose_builder, create_polysaccharides_builder, create_proteins_builder,
};
use bevy::log;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::dynamics::{Damping, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties};
use rand::Rng;

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
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    (0..2000).enumerate().for_each(|_| {
        let mut cell = Cell::default();
        cell.inject_component(burn_glucose_builder(rand::random(), rand::random()));
        cell.inject_component(CellComponentType::Internal(CellComponent {
            size: rand::random(),
            run: Box::new(move |cell: &mut CellData, dt: f32| {
                cell.base.glucose += dt * rand::random::<f32>() * 1.;
                None
            }),
        }));
        commands.spawn((CellBundle::new(
            &mut meshes,
            &mut materials,
            Vec2::new(window.width(), window.height()),
            cell,
            Vec3::new(
                rand::random::<f32>() * window.width(),
                rand::random::<f32>() * window.height(),
                0.,
            ),
        ),));
    });
}
