use bevy::{ecs::bundle::Bundle, render::color::Color};
use bevy_ascii_terminal::TileFormatter;

use crate::components::{
    player::Player,
    position::Position,
    renderable::{RenderLayer, Renderable},
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub renderable: Renderable,
    pub position: Position,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player,
            renderable: Renderable::new('@'.fg(Color::WHITE), RenderLayer::Player),
            position: Position::default(),
        }
    }
}
