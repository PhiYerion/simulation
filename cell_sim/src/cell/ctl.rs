use std::sync::Arc;

use super::cell_base::{Cell, CellComponentType, CellData};
use super::cell_bundle::{update_cell_mesh, update_cell_physics, CellBundle};
use super::cell_components::CellComponent;
use super::component_instances::{
    burn_glucose_builder, create_cell, create_cell_builder, flagella_builder, ComponentBuilderProps,
};
use super::rna::build_rna;
use super::weights::WeightList;
use bevy::log;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_rapier2d::dynamics::{Damping, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties};

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
) {
    log::info!("cells: {}", cell_zip.iter().len());
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
        let size = cell.size();
        cell.data
            .new_cells
            .drain(..)
            .enumerate()
            .for_each(|(i, new_cell)| {
                let offset =
                    Vec3::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5, 0.)
                        .normalize()
                        * f32::sqrt(size)
                        * (i + 1) as f32;
                log::info!("Cell {:?} spawned", entity);
                spawn_cell(
                    new_cell,
                    &mut commands,
                    &mut color_assets,
                    &mut mesh_assets,
                    transform.translation + offset,
                );
            });
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
    }
    let a = time.delta().as_millis();
    cell_zip.par_iter_mut().for_each(|(_, mut cell, ..)| {
        for _ in 0..(a) {
            if cell.data.base.atp <= 0.1 {
                break;
            }
            cell.update(0.01);
        }
    });
}

fn spawn_cell(
    cell: Cell,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    position: Vec3,
) {
    commands.spawn((CellBundle::new(meshes, materials, cell, position),));
}

pub fn spawn_cells(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    (0..500).enumerate().for_each(|_| {
        spawn_cell(
            create_cell(build_rna(&WeightList::default(), 1., &Vec::new())),
            &mut commands,
            &mut materials,
            &mut meshes,
            Vec3::new(
                rand::random::<f32>() * window.width(),
                rand::random::<f32>() * window.height(),
                0.,
            ),
        )
    });
}
