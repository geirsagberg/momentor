use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PixelCameraPlugin)
            .add_systems(Startup, setup_camera);
    }
}

#[derive(Component)]
struct RenderWindow;

fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows_query: Query<&Window>,
) {
    if let Ok(window) = windows_query.get_single() {
        let size = Extent3d {
            width: window.width() as u32,
            height: window.height() as u32,
            ..default()
        };
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::COPY_DST
                    | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
                label: None,
            },
            ..default()
        };
        image.resize(size);
        let image_handle = images.add(image);
        // first camera
        commands.spawn(Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        });

        let render_layer = RenderLayers::layer(1);

        commands.spawn((
            RenderWindow,
            SpriteBundle {
                texture: image_handle.clone(),
                ..default()
            },
            render_layer,
        ));

        commands.spawn((PixelCameraBundle::from_zoom(2), render_layer));
    }
}
