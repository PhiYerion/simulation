use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_xpbd_2d::components::{Collider, RigidBody};

#[derive(Component)]
pub struct Cell {
    pub speed: f32,
    pub size: f32,
    pub velocity: Vec2,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            speed: 1.,
            size: 50.,
            velocity: Vec2::new(0., 0.),
        }
    }
}

#[derive(Bundle)]
pub struct CellBundle {
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub cell: Cell,
}

impl CellBundle {
    pub fn new(meshes: &mut Assets<Mesh>, materials: &mut Assets<ColorMaterial>, window_size: Vec2) -> Self {
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

pub fn grow_cell(
    cell: &mut Cell,
    mesh: &mut Mesh2dHandle,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    growth: f32,
) {
    cell.size += growth;
    *mesh = mesh_assets
        .add(shape::Circle::new(cell.size).into())
        .into();
}

pub fn move_cell(transform: &mut Transform, cell: &mut Cell, window: &Window, vel: Vec2) {
    cell.velocity = (cell.velocity + vel).normalize() * cell.speed;
    transform.translation += cell.velocity.extend(0.);
    bound_circle_pos(&mut transform.translation, cell.size, window)
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
