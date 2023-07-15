use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{
    default, shape, Color, ColorMaterial, Commands, Component, Mesh, OnEnter, Query, Res, ResMut,
    Time, Transform, With,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::world::{CENTER_X, CENTER_Y};
use crate::GameState;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_platforms)
            .add_systems(Update, rotate_platforms);
    }
}

#[derive(Component)]
struct Platform;

fn spawn_platforms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(128., 8.))))
                .into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_xyz(CENTER_X, CENTER_Y - 100., 1.)
                .with_rotation(Quat::from_rotation_z(0.1)),
            ..default()
        },
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(64., 4.),
        Platform,
    ));
}

fn rotate_platforms(time: Res<Time>, mut query: Query<&mut Transform, With<Platform>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(time.delta_seconds() * 0.5));
    }
}
