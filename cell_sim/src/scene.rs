use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::geometry::Collider;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..Default::default()
    });
    commands
        .spawn(Collider::cuboid(window.width(), 10.))
        .insert(TransformBundle::from(Transform::from_xyz(0., 0., 0.)));
    commands
        .spawn(Collider::cuboid(window.width(), 10.))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.,
            window.height(),
            0.,
        )));
    commands
        .spawn(Collider::cuboid(10., window.height()))
        .insert(TransformBundle::from(Transform::from_xyz(0., 0., 0.)));
    commands
        .spawn(Collider::cuboid(10., window.height()))
        .insert(TransformBundle::from(Transform::from_xyz(
            window.width(),
            0.,
            0.,
        )));
}
