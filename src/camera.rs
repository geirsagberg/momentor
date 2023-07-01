use bevy::app::{App, Plugin};
use bevy::prelude::Commands;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PixelCameraPlugin)
            .add_startup_system(setup_camera)
        ;
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(PixelCameraBundle::from_zoom(4));
}
