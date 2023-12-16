#![allow(unused_parens)]

use assets::LoadingPlugin;
use background::BackgroundPlugin;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use materials::ScreenSpaceMaterial;
use platforms::PlatformsPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

use crate::animation::AnimationPlugin;

mod animation;
mod assets;
mod atlas_data;
mod background;
mod camera;
mod components;
mod debug;
mod materials;
mod music;
mod platforms;
mod player;
mod world;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            Material2dPlugin::<ScreenSpaceMaterial>::default(),
            LoadingPlugin,
            CameraPlugin,
            BackgroundPlugin,
            PlayerPlugin,
            AnimationPlugin,
            PlatformsPlugin,
            WorldPlugin,
            DebugPlugin,
            //MusicPlugin
        ));
    }
}
