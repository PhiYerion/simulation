use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::components::{RigidBody, Collider};

#[derive(Component)]
pub struct Cell {
    pub speed: f32,
    pub size: f32,
    pub velocity: Vec2,

}

impl Default for Cell {
    fn default() -> Self {
        Self {
            speed: 5.,
            size: 50.,
            velocity: Vec2::ZERO,
        }
    }
}

pub fn spawn_cell(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let a: Mesh2dHandle = meshes.add(shape::Circle::new(50.).into()).into();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(50.),
        Cell::default(),
    ));
}

pub fn grow_cell(
    mut commands: Commands,
    mut cell_bundle: Query<(&mut Mesh2dHandle, &mut Cell, &mut Transform)>,
    mut meshes_assets: ResMut<Assets<Mesh>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut mesh, mut cell, mut transform) in cell_bundle.iter_mut() {
        cell.size += 1.;
        *mesh = meshes_assets
            .add(shape::Circle::new(cell.size).into())
            .into();

        let cell_speed = cell.speed;
        let rand_speed = || {
            (rand::random::<f32>() - 0.5) * cell_speed
        };
        cell.velocity += Vec2::new(rand_speed(), rand_speed());
        cell.velocity = cell.velocity.normalize() * cell_speed;
        transform.translation += cell.velocity.extend(0.);
        bound_circle_pos(&mut transform.translation, cell.size, window.get_single().unwrap())
    }
}

pub fn move_all_cells(
    cells: Query<&Cell>,
    mut tranforms: Query<&mut Transform, With<Cell>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut transform, cell) in tranforms.iter_mut().zip(cells.iter()) {
        move_cell(&mut transform, cell, window.single());
    }
}

fn move_cell(transform: &mut Transform, cell: &Cell, window: &Window) {
    bound_rectangle_pos(&mut transform.translation, Vec2::ZERO, window);
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

fn bound_rectangle_pos(pos: &mut Vec3, scale: Vec2, window: &Window) {
    let half_scale = scale / 2.;
    let min = half_scale;
    let max = Vec2::new(window.width(), window.height()) - half_scale;

    if pos.x < min.x {
        pos.x = min.x;
    } else if pos.y < min.y {
        pos.y = min.y;
    }

    if pos.x > max.x {
        pos.x = max.x;
    } else if pos.y > max.y {
        pos.y = max.y;
    }
}
