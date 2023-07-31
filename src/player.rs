#![allow(unused_parens)]

use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::VirtualDPad;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::{Actionlike, InputManagerBundle};

use crate::animation::Animation;
use crate::assets::TextureAssets;
use crate::atlas_data::AnimationSpriteSheetMeta;
use crate::world::{CENTER_X, CENTER_Y};
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_character);
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum PlayerAction {
    Move,
    Aim,
    Jump,
    Shoot,
}

fn move_character(
    mut query: Query<
        (
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
            &ActionState<PlayerAction>,
        ),
        With<Player>,
    >,
) {
    for (mut controller, output, action_state) in &mut query {
        let axis_pair = action_state
            .axis_pair(PlayerAction::Move)
            .unwrap_or_default();
        let speed = 2.;

        let translation = output
            .map(|it| it.effective_translation)
            .unwrap_or_default();

        let x = axis_pair.x() * speed;
        let y = (translation.y - 0.1).max(-3.);
        controller.translation = Some(Vec2::new(x, y));
    }
}

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    animated_sprite_sheet_assets: Res<Assets<AnimationSpriteSheetMeta>>,
) {
    let cyborg = animated_sprite_sheet_assets.get(&textures.cyborg).unwrap();
    let mut animation = Animation::new(cyborg.animation_frame_duration, cyborg.animations.clone());
    animation.play("idle", true);
    let mut input_map = InputMap::default();
    input_map
        .insert(VirtualDPad::wasd(), PlayerAction::Move)
        .insert(KeyCode::Space, PlayerAction::Jump);

    commands.spawn((
        Player,
        SpriteSheetBundle {
            texture_atlas: cyborg.atlas_handle.clone(),
            transform: Transform::from_xyz(CENTER_X, CENTER_Y, 3.),
            ..default()
        },
        animation,
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(8., 16.),
        KinematicCharacterController::default(),
    ));
}
