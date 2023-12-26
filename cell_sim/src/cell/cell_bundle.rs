use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_xpbd_2d::components::{Collider, RigidBody};

use super::cell_base::Cell;

#[derive(Bundle)]
pub struct CellBundle {
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub cell: Cell,
}

impl CellBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
        window_size: Vec2,
    ) -> Self {
        Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_xyz(window_size.x / 2., window_size.y / 2., 0.),
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(50.),
            cell: Cell::default(),
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

    let r = cell.atp / cell.atp_storage;
    let g = cell.food / cell.food_storage;
    let new_color = ColorMaterial::from(Color::rgb(r, g, 0.));
    *color = color_assets.add(new_color);
}

pub fn move_cell(transform: &mut Transform, cell: &mut Cell, window: &Window, vel: Vec2) {
    cell.velocity = (cell.velocity + vel).normalize() * cell.speed;
    transform.translation += cell.velocity.extend(0.);
    bound_circle_pos(&mut transform.translation, cell.size(), window)
}

fn bound_circle_pos(pos: &mut Vec3, radius: f32, window: &Window) {
    let min = radius;
    let max = Vec2::new(window.width(), window.height()) - radius;

    if pos.x < min {
        pos.x = min;
    } else if pos.x > max.x {
        pos.x = max.x;
    }

    if pos.y > max.y {
        pos.y = max.y;
    } else if pos.y < min {
        pos.y = min;
    }
}
