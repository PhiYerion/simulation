use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::{log, prelude::*};
use bevy_rapier2d::dynamics::{RigidBody, Velocity, Damping};
use bevy_rapier2d::geometry::{Collider, Restitution, ColliderMassProperties};
use bevy_rapier2d::rapier::dynamics::RigidBodyDamping;

use super::cell_base::Cell;

#[derive(Bundle)]
pub struct CellBundle {
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub collider: Collider,
    pub collider_mass_properties: ColliderMassProperties,
    pub cell: Cell,
    pub damping: Damping,
    pub resitution: Restitution,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
}

impl CellBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        window_size: Vec2,
    ) -> Self {
        Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(100.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(window_size.x / 2., window_size.y / 2., 0.),
                ..default()
            },
            collider: Collider::ball(100.),
            collider_mass_properties: ColliderMassProperties::Density(1.),
            damping: Damping { 
                linear_damping: 1.,
                angular_damping: 1.,
            },
            resitution: Restitution::coefficient(0.5),
            rigid_body: RigidBody::Dynamic,
            cell: Cell::default(),
            velocity: Velocity::default(),
        }
    }
}

pub fn update_cell_mesh(
    cell: &mut Cell,
    collider: &mut Collider,
    mesh: &mut Mesh2dHandle,
    color: &mut Handle<ColorMaterial>,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    color_assets: &mut ResMut<Assets<ColorMaterial>>,
) {
    *mesh = mesh_assets
        .add(shape::Circle::new(cell.size()).into())
        .into();
    *collider = Collider::ball(cell.size());
    log::info!("cell size: {}", cell.size());

    let r = cell.atp / cell.atp_storage;
    let g = cell.food / cell.food_storage;
    let new_color = ColorMaterial::from(Color::rgb(r, g, 0.));
    *color = color_assets.add(new_color);
}

pub fn move_cell(velocity: &mut Velocity, cell: &mut Cell, window: &Window, vel: Vec2) {
    const DRAG: Vec2 = Vec2::new(0.5, 0.5);
    cell.velocity += vel * cell.speed;
    velocity.linvel = cell.velocity - DRAG;
    velocity.angvel = 0.;
}
