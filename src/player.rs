use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::VirtualDPad;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::{Actionlike, InputManagerBundle};

use crate::animation::Animation;
use crate::assets::TextureAssets;
use crate::atlas_data::AnimationSpriteSheetMeta;
use crate::components::facing::{Facing, FacingDirection};
use crate::world::{CENTER_X, CENTER_Y};
use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, (move_player, animate_player).chain());
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum PlayerAction {
    Move,
    Aim,
    Jump,
    Shoot,
}

fn animate_player(mut query: Query<(&mut Animation, &ActionState<PlayerAction>), With<Player>>) {
    for (mut animation, action_state) in &mut query {
        let axis_pair = action_state
            .axis_pair(PlayerAction::Move)
            .unwrap_or_default();
        let x = axis_pair.x();
        let y = axis_pair.y();

        if let Some(anim) = &animation.current_animation {
            match anim.as_str() {
                "idle" => {
                    if x != 0. || y != 0. {
                        animation.play("run", true);
                    }
                }
                "run" => {
                    if x == 0. && y == 0. {
                        animation.play("idle", true);
                    }
                }
                _ => {}
            }
        }
    }
}

fn move_player(mut query: Query<(&ActionState<PlayerAction>, &mut Facing), With<Player>>) {
    for (action_state, mut facing) in &mut query {
        let axis_pair = action_state
            .axis_pair(PlayerAction::Move)
            .unwrap_or_default();
        let speed = 2.;

        let x = axis_pair.x() * speed;

        if (x.abs() > 0.) {
            if x < 0. && facing.direction == FacingDirection::Right {
                facing.direction = FacingDirection::Left;
            } else if x > 0. && facing.direction == FacingDirection::Left {
                facing.direction = FacingDirection::Right;
            }
        }
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
            sprite: TextureAtlasSprite {
                anchor: Anchor::Custom((0.0, -0.1).into()),
                ..default()
            },
            ..default()
        },
        animation,
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        },
        RigidBody::Dynamic,
        Collider::cuboid(8., 16.),
        Facing::default(),
    ));
}
