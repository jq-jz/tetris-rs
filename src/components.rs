use bevy::prelude::*;

#[derive(Component)]
pub struct Block;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum UiText {
    Score,
}
