use bevy::{ecs::bundle::Bundle, render::color::Color};
use bevy_ascii_terminal::TileFormatter;

use crate::components::{
    ai::Ai,
    energy::Energy,
    name::Name,
    position::Position,
    renderable::{RenderLayer, Renderable},
};

#[derive(Bundle, Clone)]
pub struct GoblinBundle {
    pub renderable: Renderable,
    pub position: Position,
    pub energy: Energy,
    pub ai: Ai,
    pub name: Name,
}

impl Default for GoblinBundle {
    fn default() -> Self {
        GoblinBundle {
            renderable: Renderable::new('g'.fg(Color::RED), RenderLayer::Monsters),
            position: Position::default(),
            energy: Energy::new(2),
            ai: Ai::Monster,
            name: Name::new("goblin"),
        }
    }
}
