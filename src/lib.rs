use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};

use assets::LoadingPlugin;
use background::BackgroundPlugin;
use camera::CameraPlugin;
use music::MusicPlugin;

mod assets;
mod camera;
mod background;
mod music;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_plugin(Material2dPlugin::<ScreenSpaceMaterial>::default())
            .add_plugin(LoadingPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(BackgroundPlugin)
            .add_plugin(MusicPlugin)
            .add_system(spawn_platforms.in_schedule(OnEnter(GameState::Playing)))
        ;
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "499a11e9-7a0e-4476-b890-a90c8bf2e19a"]
pub struct ScreenSpaceMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Option<Handle<Image>>,
}

impl Material2d for ScreenSpaceMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/screen_space.wgsl".into()
    }
}

fn spawn_platforms(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // commands.spawn(
    //     MaterialMesh2dBundle {
    //         mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(100., 100.)))).into(),
    //         material: materials.add(ColorMaterial::from(Color::RED)),
    //         transform: Transform::from_xyz(0., 0., 1.),
    //         ..default()
    //     }
    // );
}
