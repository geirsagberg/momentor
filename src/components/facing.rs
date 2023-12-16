use bevy::{
    prelude::{Component, Deref, DerefMut},
    reflect::Reflect,
};

#[derive(PartialEq, Eq, Clone, Debug, Reflect, Default)]
pub enum FacingDirection {
    Left,
    #[default]
    Right,
}

#[derive(Component, PartialEq, Eq, Clone, Debug, Reflect, Default, DerefMut, Deref)]
pub struct Facing {
    pub direction: FacingDirection,
}
