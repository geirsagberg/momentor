use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use assets::LoadingPlugin;
use crate::assets::TextureAssets;

mod assets;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()

            .add_plugin(LoadingPlugin)
            .add_plugin(CameraPlugin)
            .add_systems((spawn_platforms, spawn_background).in_schedule(OnEnter(GameState::Playing)))
        ;
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PixelCameraPlugin)
            .add_startup_system(setup_camera)
        ;
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_zoom(4));
}

fn spawn_platforms(mut commands: Commands) {
    // commands.spawn()
}

fn spawn_background(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    commands.spawn(
        SpriteBundle {
            texture: texture_assets.background_1.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        }
    );
    commands.spawn(
        SpriteBundle {
            texture: texture_assets.background_2.clone(),
            transform: Transform::from_xyz(0., 0., 0.1),
            ..default()
        }
    );
    commands.spawn(
        SpriteBundle {
            texture: texture_assets.background_3.clone(),
            transform: Transform::from_xyz(0., 0., 0.2),
            ..default()
        }
    );
    commands.spawn(
        SpriteBundle {
            texture: texture_assets.background_4.clone(),
            transform: Transform::from_xyz(0., 0., 0.3),
            ..default()
        }
    );
    commands.spawn(
        SpriteBundle {
            texture: texture_assets.background_5.clone(),
            transform: Transform::from_xyz(0., 0., 0.4),
            ..default()
        }
    );
}
