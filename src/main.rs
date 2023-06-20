use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;
use momentor::MainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Momentor".to_string(),
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(RapierDebugRenderPlugin::default().disabled())
        .add_plugin(EguiPlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(MainPlugin)
        .run();
}
