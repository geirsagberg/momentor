use bevy::app::{App, Plugin, Update};
use bevy::asset::Assets;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{
    default, shape, Color, ColorMaterial, Commands, Component, Mesh, OnEnter, Query, Res, ResMut,
    Time, Transform, With,
};
use bevy::sprite::MaterialMesh2dBundle;
use bevy_xpbd_2d::prelude::*;

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
    let mut platform = |size: Vec2,
                        center: Vec2|
     -> (
        MaterialMesh2dBundle<ColorMaterial>,
        RigidBody,
        Collider,
        Platform,
    ) {
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(size))).into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_xyz(center.x, center.y, 1.),
                ..default()
            },
            RigidBody::Static,
            Collider::cuboid(size.x / 2., size.y / 2.),
            Platform,
        )
    };

    commands.spawn(platform(
        Vec2::new(512., 8.),
        Vec2::new(CENTER_X, CENTER_Y - 100.),
    ));
    commands.spawn(platform(
        Vec2::new(128., 8.),
        Vec2::new(CENTER_X + 100., CENTER_Y + 100.),
    ));
}

#[derive(Component)]
pub struct Rotating {
    pub speed: f32,
}

fn rotate_platforms(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Rotating), With<Platform>>,
) {
    for (mut transform, rotating) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(time.delta_seconds() * rotating.speed));
    }
}
