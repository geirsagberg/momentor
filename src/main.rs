#![allow(unused_parens)]

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_framepace::FramepacePlugin;

use bevy_xpbd_2d::plugins::PhysicsDebugPlugin;
use bevy_xpbd_2d::prelude::PhysicsPlugins;
use momentor::MainPlugin;

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
            EguiPlugin,
            MainPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .run();
}
