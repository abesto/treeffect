use bevy::{ecs::bundle::Bundle, render::color::Color};
use bevy_ascii_terminal::TileFormatter;

use crate::components::{
    ai::Ai,
    energy::Energy,
    name::Name,
    position::Position,
    renderable::{RenderLayer, Renderable},
};

#[derive(Bundle)]
pub struct DogBundle {
    pub renderable: Renderable,
    pub position: Position,
    pub energy: Energy,
    pub ai: Ai,
    pub name: Name,
}

impl Default for DogBundle {
    fn default() -> Self {
        DogBundle {
            renderable: Renderable::new('d'.fg(Color::WHITE), RenderLayer::Monsters),
            position: Position::default(),
            energy: Energy::new(1),
            ai: Ai::Dog,
            name: Name::new("Fluffy"),
        }
    }
}
