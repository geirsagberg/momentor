use bevy::app::{App, Plugin, Update};
use bevy::input::Input;
use bevy::prelude::{KeyCode, Res, ResMut};
use bevy_xpbd_2d::prelude::debug::PhysicsDebugConfig;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_debug);
    }
}

fn toggle_debug(
    mut rapier_context: ResMut<PhysicsDebugConfig>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        rapier_context.enabled = !rapier_context.enabled;
    }
}
