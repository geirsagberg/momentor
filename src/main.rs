#![allow(unused_parens)]

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_framepace::{FramepacePlugin, FramepaceSettings};
use bevy_rapier2d::prelude::*;

use momentor::MainPlugin;

use crate::screen_diags::ScreenDiagsTextPlugin;

mod screen_diags;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Momentor".to_string(),
                        resolution: (1920., 1080.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            FramepacePlugin,
            ScreenDiagsTextPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            RapierDebugRenderPlugin::default().disabled(),
            EguiPlugin,
            MainPlugin,
        ))
        .insert_resource(FramepaceSettings {
            limiter: bevy_framepace::Limiter::from_framerate(60.),
            ..Default::default()
        })
        .run();
}
