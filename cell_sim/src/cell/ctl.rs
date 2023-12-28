use super::cell_base::{Cell, CellComponent, CellComponentType, CellData};
use super::cell_bundle::{update_cell_mesh, update_cell_physics, CellBundle};
use super::cell_components::{
    burn_glucose_builder, create_polysaccharides_builder, create_proteins_builder, flagella_builder,
};
use bevy::log;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::dynamics::{Damping, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties};
use rand::Rng;

type CellZip<'a> = (
    Entity,
    &'a mut Cell,
    &'a mut Collider,
    &'a mut Velocity,
    &'a mut Mesh2dHandle,
    &'a mut Handle<ColorMaterial>,
    &'a mut ColliderMassProperties,
    &'a mut Damping,
);

pub fn update_all_cells(
    mut commands: Commands,
    mut cell_zip: Query<CellZip>,
    time: Res<Time>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut color_assets: ResMut<Assets<ColorMaterial>>,
) {
    for (
        entity,
        mut cell,
        mut collider,
        mut velocity,
        mut mesh,
        mut color,
        mut collider_mass_properties,
        mut damping,
    ) in cell_zip.iter_mut()
    {
        if cell.data.base.atp <= 0.1 {
            commands.entity(entity).despawn();
            log::info!("Cell {:?} died", entity);
        }
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
            &mut velocity,
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
    (0..1000).enumerate().for_each(|_| {
        let mut cell = Cell::default();
        cell.inject_component(burn_glucose_builder(rand::random(), rand::random()));
        cell.inject_component(flagella_builder(rand::random::<f32>() * 10., rand::random::<f32>() * 100.));
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
