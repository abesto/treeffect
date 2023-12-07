use bevy::{ecs::bundle::Bundle, render::color::Color};
use bevy_ascii_terminal::TileFormatter;

use crate::components::{
    ai::Ai,
    energy::Energy,
    position::Position,
    renderable::{RenderLayer, Renderable},
};

#[derive(Bundle, Clone)]
pub struct OrcBundle {
    pub renderable: Renderable,
    pub position: Position,
    pub energy: Energy,
    pub ai: Ai,
}

impl Default for OrcBundle {
    fn default() -> Self {
        OrcBundle {
            renderable: Renderable::new('o'.fg(Color::RED), RenderLayer::Monsters),
            position: Position::default(),
            energy: Energy::new(2),
            ai: Ai::Monster,
        }
    }
}
