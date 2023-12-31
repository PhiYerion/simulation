use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::{log, prelude::*};
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

const CELL_SIZE_MODIFIER: f32 = 0.2;

impl CellBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        cell: Cell,
        pos: Vec3,
    ) -> Self {
        Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(cell.size() * CELL_SIZE_MODIFIER).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..default()
            },
            collider: Collider::ball(cell.size() * CELL_SIZE_MODIFIER),
            collider_mass_properties: ColliderMassProperties::Density(1.),
            damping: Damping {
                linear_damping: 1.,
                angular_damping: 1.,
            },
            resitution: Restitution::coefficient(0.5),
            rigid_body: RigidBody::Dynamic,
            cell,
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
        .add(shape::Circle::new(cell.size() * CELL_SIZE_MODIFIER).into())
        .into();

    let r = cell.data.base.atp.min(1.);
    let g = cell.data.base.glucose.min(1.);
    let new_color = ColorMaterial::from(Color::rgb(r, g, 0.));
    *color = color_assets.add(new_color);
}

pub fn update_cell_physics(
    cell: &Cell,
    collider: &mut Collider,
    collider_mass_properties: &mut ColliderMassProperties,
    damping: &mut Damping,
    velocity: &mut Velocity,
) {
    velocity.linvel = cell.data.velocity;
    *collider = Collider::ball(cell.size() * CELL_SIZE_MODIFIER);
    *collider_mass_properties = ColliderMassProperties::Density(cell.size());
    *damping = Damping {
        linear_damping: cell.data.speed * cell.data.speed / 4.,
        angular_damping: cell.data.speed * cell.data.speed / 4.,
    };
}
