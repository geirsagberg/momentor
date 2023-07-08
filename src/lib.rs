use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Anchor, Material2d, Material2dPlugin, MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use assets::LoadingPlugin;
use background::BackgroundPlugin;
use camera::CameraPlugin;
use music::MusicPlugin;

use crate::animation::{Animation, AnimationPlugin};
use crate::assets::TextureAssets;
use crate::atlas_data::AnimationSpriteSheetMeta;

mod atlas_data;
mod animation;
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
            // .add_plugin(MusicPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(AnimationPlugin)
            .add_system(spawn_platforms.in_schedule(OnEnter(GameState::Playing)))
        ;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
        ;
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PlayerAction {
    Move,
    Aim,
    Jump,
    Shoot,
}

fn spawn_player(mut commands: Commands,
                textures: Res<TextureAssets>, animated_sprite_sheet_assets: Res<Assets<AnimationSpriteSheetMeta>>) {
    let cyborg = animated_sprite_sheet_assets.get(&textures.cyborg).unwrap();
    let mut animation = Animation::new(cyborg.animation_frame_duration, cyborg.animations.clone());
    animation.play("idle", true);
    let mut input_map = InputMap::default();
    input_map
        .insert(VirtualDPad::wasd(), PlayerAction::Move)
        .insert(KeyCode::Space, PlayerAction::Jump);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: cyborg.atlas_handle.clone(),
        transform: Transform::from_xyz(0., 0., 3.),
        ..default()
    })
        .insert(animation)
        .insert(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        })
    // .insert(RigidBody::KinematicPositionBased)
    // .insert(Collider::capsule_y(8. , 8.))
    // .insert(KinematicCharacterController::default())
    ;
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
    commands.spawn(
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(128., 8.)))).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_xyz(-100., -100., 1.).with_rotation(Quat::from_rotation_z(0.1)),
            ..default()
        }
    );
}
