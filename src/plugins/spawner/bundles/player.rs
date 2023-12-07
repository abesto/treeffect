use bevy::{ecs::bundle::Bundle, render::color::Color};
use bevy_ascii_terminal::TileFormatter;

use crate::components::{
    energy::Energy,
    name::Name,
    player::Player,
    position::Position,
    renderable::{RenderLayer, Renderable},
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub renderable: Renderable,
    pub position: Position,
    pub energy: Energy,
    pub name: Name,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player,
            renderable: Renderable::new('@'.fg(Color::WHITE), RenderLayer::Player),
            position: Position::default(),
            energy: Energy::new(1000),
            name: Name::new("Player"),
        }
    }
}
