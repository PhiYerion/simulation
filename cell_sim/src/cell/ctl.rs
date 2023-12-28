use std::sync::Arc;

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
    &'a Transform,
);

pub fn update_all_cells(
    mut commands: Commands,
    mut cell_zip: Query<CellZip>,
    time: Res<Time>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut color_assets: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    for (
        entity,
        mut cell,
        mut collider,
        mut velocity,
        mut mesh,
        mut color,
        mut collider_mass_properties,
        mut damping,
        transform,
    ) in cell_zip.iter_mut()
    {
        if cell.data.base.atp <= 0.1 {
            commands.entity(entity).despawn();
            log::info!("Cell {:?} died", entity);
        }
        let internal_components = cell.internal_components.clone();
        let membrane_components = cell.membrane_components.clone();
        cell.data.new_cells.drain(..).for_each(|new_cell| {
            log::info!("Cell {:?} spawned", entity);
            let mut new_cell = new_cell;
            new_cell.internal_components = internal_components.clone();
            new_cell.membrane_components = membrane_components.clone();
            spawn_cell(
                new_cell,
                &mut commands,
                &mut color_assets,
                &mut mesh_assets,
                Vec2::new(
                    transform.translation.x + rand::random::<f32>(),
                    transform.translation.y + rand::random::<f32>(),
                ),
            );
        });
        cell.data.new_cells.clear();
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

fn create_basic_cell() -> Cell {
    let mut cell = Cell::default();
    cell.inject_component(burn_glucose_builder(rand::random(), rand::random()));
    cell.inject_component(flagella_builder(
        rand::random::<f32>() * 10.,
        rand::random::<f32>() * 1.,
    ));

    cell.inject_component(CellComponentType::Internal(CellComponent {
        size: 1.,
        run: Arc::new(move |data: &mut CellData, dt: f32| {
            data.base.glucose += 1. * dt;
            data.base.atp -= data.base.size() * data.base.size() / 200.;
            if data.base.atp >= 15. {
                data.base.atp -= 10.;
                let new_cell = create_basic_cell();
                data.new_cells.push(new_cell);
            }

            None
        }),
    }));

    cell
}

fn spawn_cell(
    cell: Cell,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: Vec2,
) {
    commands.spawn((CellBundle::new(
        meshes,
        materials,
        cell,
        Vec3::new(position.x, position.y, 0.),
    ),));
}

pub fn spawn_cells(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    (0..1000).enumerate().for_each(|_| {
        let mut cell = create_basic_cell();
        spawn_cell(
            cell,
            &mut commands,
            &mut materials,
            &mut meshes,
            Vec2::new(
                rand::random::<f32>() * window.width(),
                rand::random::<f32>() * window.height(),
            ),
        )
    });
}
