use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::dynamics::{Damping, RigidBody, Velocity};
use bevy_rapier2d::geometry::{Collider, ColliderMassProperties, Restitution};

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
    mesh: &mut Mesh2dHandle,
    color: &mut Handle<ColorMaterial>,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    color_assets: &mut ResMut<Assets<ColorMaterial>>,
) {
    *mesh = mesh_assets
        .add(shape::Circle::new(cell.size()).into())
        .into();

    let r = cell.data.atp / cell.data.atp_storage;
    let g = cell.data.food / cell.data.food_storage;
    let new_color = ColorMaterial::from(Color::rgb(r, g, 0.));
    *color = color_assets.add(new_color);
}

pub fn update_cell_physics(
    cell: &Cell,
    collider: &mut Collider,
    collider_mass_properties: &mut ColliderMassProperties,
    damping: &mut Damping,
) {
    *collider = Collider::ball(cell.size());
    *collider_mass_properties = ColliderMassProperties::Density(cell.size());
    *damping = Damping {
        linear_damping: cell.speed * cell.speed / 4.,
        angular_damping: cell.speed * cell.speed / 4.,
    };
}

pub fn move_cell(velocity: &mut Velocity, cell: &mut Cell, vel: Vec2) {
    cell.velocity += vel * cell.speed;
    velocity.linvel = cell.velocity;
    velocity.angvel = 0.;
}
