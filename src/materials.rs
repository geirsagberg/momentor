use bevy::asset::{Asset, Handle};
use bevy::prelude::{Image, Reflect};
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Reflect, Asset)]
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

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Reflect, Asset)]
#[uuid = "cce622bd-a45a-40f3-a6e4-468aa0e6ba85"]
pub struct ParallaxMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Option<Handle<Image>>,

    #[uniform(2)]
    pub speed: f32,
}

impl Material2d for ParallaxMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/parallax.wgsl".into()
    }
}
