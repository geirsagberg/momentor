use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, default, Image, IntoSystemAppConfig, Mesh, OnEnter, Res, ResMut, SpriteBundle, Transform};
use bevy::asset::{Assets, Handle};
use bevy::math::Vec3;
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle};
use bevy::prelude::shape::Quad;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::reflect::TypeUuid;
use crate::assets::TextureAssets;
use crate::GameState;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(Material2dPlugin::<ParallaxMaterial>::default())
            .add_system(spawn_background.in_schedule(OnEnter(GameState::Playing)))
        ;
    }
}

fn spawn_background(
    mut commands: Commands, texture_assets: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParallaxMaterial>>,
    images: Res<Assets<Image>>,
) {
    let image_size = images.get(&texture_assets.background_1).unwrap().size();
    let mesh: Mesh2dHandle = meshes.add(Mesh::from(Quad::new(image_size))).into();

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
        MaterialMesh2dBundle {
            mesh: mesh.clone(),
            material: materials.add(ParallaxMaterial {
                texture: Some(texture_assets.background_3.clone()),
                speed: 0.01,
            }),
            transform: Transform::from_xyz(0., 0., 0.2),
            ..default()
        }
    );
    commands.spawn(
        MaterialMesh2dBundle {
            mesh: mesh.clone(),
            material: materials.add(ParallaxMaterial {
                texture: Some(texture_assets.background_4.clone()),
                speed: 0.02,
            }),
            transform: Transform::from_xyz(0., 0., 0.3),
            ..default()
        }
    );

    commands.spawn(
        MaterialMesh2dBundle {
            mesh: mesh.clone(),
            material: materials.add(ParallaxMaterial {
                texture: Some(texture_assets.background_5.clone()),
                speed: 0.03,
            }),
            transform: Transform::from_xyz(0., 0., 0.4),
            ..default()
        }
    );
}


#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "cce622bd-a45a-40f3-a6e4-468aa0e6ba85"]
pub struct ParallaxMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Option<Handle<Image>>,

    #[uniform(2)]
    speed: f32,
}

impl Material2d for ParallaxMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/parallax.wgsl".into()
    }
}
